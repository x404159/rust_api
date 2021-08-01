use crate::errors::ServiceError;
use crate::models::user::SlimUser;
use crate::routes::users;
use crate::{db::Pool, utils::is_admin};

use actix_identity::Identity;
use actix_web::{
    error::BlockingError,
    web::{self, ServiceConfig},
    HttpResponse,
};
use diesel::{EqAll, QueryDsl, RunQueryDsl};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserData {
    name: String,
    email: String,
    password: String,
}

//routes
pub fn users_route_config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .route(web::post().to(users::post_user))
            .route(web::get().to(users::get_users)),
    )
    .route("/users/{id}", web::patch().to(users::change_account_type));
}

//route handles
//POST /users
pub async fn post_user(
    user_data: web::Json<UserData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || insert_user(user_data.into_inner(), pool)).await;

    match res {
        //login after user creation
        Ok(user) => Ok(HttpResponse::Created().body(serde_json::json!({ "email": user.email }))),
        Err(e) => match e {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}

//GET /users
pub async fn get_users(id: Identity, pool: web::Data<Pool>) -> Result<HttpResponse, ServiceError> {
    let _ = crate::utils::is_admin(&id)?;
    let res = web::block(move || get_all_users(pool)).await;
    match res {
        Ok(user) => Ok(HttpResponse::Ok().json(&user)),
        Err(e) => match e {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}

//PATCH /users/{id}
pub async fn change_account_type(
    id: Identity,
    user_id: web::Path<String>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let _ = is_admin(&id)?;
    let user_id = match user_id.into_inner().parse::<i64>() {
        Ok(v) => v,
        Err(_) => return Err(ServiceError::BadRequest("invalid user id".to_owned())),
    };
    let res = web::block(move || change_account(user_id, pool)).await;

    match res {
        Ok(s) => Ok(HttpResponse::Ok().json(serde_json::json!({ "msg": s }))),
        Err(e) => match e {
            BlockingError::Error(service_err) => Err(service_err),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}

//route handles helper function
fn change_account(user_id: i64, pool: web::Data<Pool>) -> Result<String, ServiceError> {
    use crate::schema::users::dsl::{clearance, users};
    let conn = &pool.get().unwrap();

    let mut return_string = String::new();
    let target = users.find(user_id);
    let current_clearance = target.select(clearance).get_result::<bool>(conn)?;
    if current_clearance {
        return_string.push_str("change account type from admin to normal user");
        let _ = diesel::update(target)
            .set(clearance.eq_all(false))
            .execute(conn)?;
    } else {
        return_string.push_str("change account type from normal user to admin");
        let _ = diesel::update(target)
            .set(clearance.eq_all(true))
            .execute(conn)?;
    }
    Ok(return_string)
}

fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<SlimUser>, ServiceError> {
    use crate::models::user::User;
    use crate::schema::users::dsl::users;
    let conn = &pool.get().unwrap();
    let all_users = users.load::<User>(conn)?;
    Ok(all_users.into_iter().map(|u| u.into()).collect())
}

fn insert_user(user_data: UserData, pool: web::Data<Pool>) -> Result<SlimUser, ServiceError> {
    use crate::models::user::{User, UserInsert};
    use crate::schema::users::dsl::users;
    let password = crate::utils::hash_password(&user_data.password)?;

    let new_user = UserInsert::from_details(user_data.name, user_data.email, password);

    let conn = &pool.get().unwrap();
    let inserted_user = diesel::insert_into(users)
        .values(&new_user)
        .get_result::<User>(conn)?;
    Ok(inserted_user.into())
}
