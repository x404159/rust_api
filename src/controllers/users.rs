use actix_web::{web, HttpRequest, HttpResponse};

use crate::{
    db::db::Pool,
    errors::ServiceError,
    models::{dbmethods, user::UserData},
    utils::parse_request,
};

//route handles
//POST /users
pub async fn post_user(
    user_data: web::Json<UserData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let user = dbmethods::insert_user(user_data.into_inner(), pool)?;
    Ok(HttpResponse::Created().body(serde_json::json!({ "email": user.email })))
}

//GET /users
pub async fn get_users(
    pool: web::Data<Pool>,
    req: HttpRequest,
) -> Result<HttpResponse, ServiceError> {
    let (_, clearance) = parse_request(req);
    if !(clearance == "admin") {
        return Err(ServiceError::Unauthorized);
    }
    let users = dbmethods::get_all_users(pool)?;
    Ok(HttpResponse::Ok().json(&users))
}

//PATCH /users/{id}
pub async fn change_account_type(
    user_id: web::Path<String>,
    pool: web::Data<Pool>,
    req: HttpRequest,
) -> Result<HttpResponse, ServiceError> {
    let (clearance, _) = parse_request(req);
    if !(clearance == "admin") {
        return Err(ServiceError::Unauthorized);
    }
    let user_id = match user_id.into_inner().parse::<i64>() {
        Ok(v) => v,
        Err(_) => return Err(ServiceError::BadRequest("invalid user id".to_owned())),
    };
    let msg = dbmethods::change_account(user_id, pool)?;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "msg": msg })))
}
