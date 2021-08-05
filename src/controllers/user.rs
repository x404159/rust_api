use actix_web::HttpRequest;
use actix_web::{web, HttpResponse};

use crate::{
    db::db::Pool,
    errors::ServiceError,
    models::{
        dbmethods,
        user::{FindBy, SlimUser, UserChange},
    },
    utils::parse_request,
};

//route handlers
//GET /user
pub async fn get_me(pool: web::Data<Pool>, req: HttpRequest) -> Result<HttpResponse, ServiceError> {
    let (email, _) = parse_request(req);
    let user = dbmethods::find_by(FindBy::Email(email), pool)?;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "id": user.id, "email": user.email, "joined": user.created_at.date() , "name": user.name, "admin": user.clearance })))
}

//GET /user/{id}
pub async fn get_user_by_id(
    id: web::Path<String>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let id = match id.into_inner().parse::<i64>() {
        Ok(v) => v,
        Err(_) => return Err(ServiceError::BadRequest("invalid id".to_owned())),
    };
    let user = dbmethods::find_by(FindBy::Id(id), pool)?;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "id": user.id, "email": user.email })))
}

// PATCH /user
pub async fn update_user(
    updates: web::Json<UserChange>,
    pool: web::Data<Pool>,
    req: HttpRequest,
) -> Result<HttpResponse, ServiceError> {
    let (email, clearance) = parse_request(req);
    let updates = updates.into_inner();
    let clearance = if clearance == "admin" { true } else { false };
    let user = SlimUser { email, clearance };
    let changed = dbmethods::user_update(user, updates, pool)?;

    Ok(HttpResponse::Ok().json(changed))
}

//DELETE /user
pub async fn remove_account(
    pool: web::Data<Pool>,
    req: HttpRequest,
) -> Result<HttpResponse, ServiceError> {
    let (email, _) = parse_request(req);
    let b = dbmethods::delete_account(email, pool)?;

    if b {
        Ok(HttpResponse::Ok().json(serde_json::json!({ "msg": "account deleted successfully" })))
    } else {
        Ok(HttpResponse::Ok().json(serde_json::json!({ "msg": "could not delete account" })))
    }
}

pub async fn test_route(pool: web::Data<Pool>) -> Result<HttpResponse, ServiceError> {
    let users = dbmethods::test_raw(&pool)?;
    Ok(HttpResponse::Ok().json(&users))
}
