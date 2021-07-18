use crate::db::Pool;
use crate::errors::ServiceError;
use crate::models::user::SlimUser;
use actix_identity::Identity;
use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::RunQueryDsl;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserData {
    name: String,
    email: String,
    password: String,
}

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

pub async fn get_users(id: Identity, pool: web::Data<Pool>) -> Result<HttpResponse, ServiceError> {
    let _ = crate::utils::is_admin(id)?;
    let res = web::block(move || get_all_users(pool)).await;
    match res {
        Ok(user) => Ok(HttpResponse::Ok().json(&user)),
        Err(e) => match e {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
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
