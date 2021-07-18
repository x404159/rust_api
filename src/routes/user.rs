use actix_identity::Identity;
use actix_web::error::BlockingError;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;

use crate::db::Pool;
use crate::errors::ServiceError;
use crate::models::user::{SlimUser, User};

pub enum FindBy {
    Email(String),
    Id(i64),
}

pub async fn get_me(id: Identity, pool: web::Data<Pool>) -> Result<HttpResponse, ServiceError> {
    if let None = id.identity() {
        return Err(ServiceError::Unauthorized);
    }
    let user = id.identity().unwrap();
    let user: SlimUser = serde_json::from_str(&user).unwrap();
    let res = web::block(|| find_by(FindBy::Email(user.email), pool)).await;
    match res {
        Ok(user) => {
            let user = serde_json::json!({ "id": user.id, "email": user.email, "joined": user.created_at.date() , "name": user.name, "admin": user.clearance });
            Ok(HttpResponse::Ok().json(&user))
        }
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}

pub async fn get_user_by_id(
    id: web::Path<String>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let id = match id.into_inner() {
        s => match s.parse::<i64>() {
            Ok(v) => v,
            Err(_) => return Err(ServiceError::BadRequest("invalid id".to_owned())),
        },
    };
    let res = web::block(move || find_by(FindBy::Id(id), pool)).await;
    match res {
        Ok(user) => {
            let user = serde_json::json!({ "id": user.id, "email": user.email });
            Ok(HttpResponse::Ok().json(&user))
        }
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}

fn find_by(data: FindBy, pool: web::Data<Pool>) -> Result<User, ServiceError> {
    use crate::schema::users::dsl::{email, id, users};
    let conn = &pool.get().unwrap();
    let mut user;
    match data {
        FindBy::Email(e) => {
            user = users.filter(email.eq(&e)).get_results::<User>(conn)?;
        }
        FindBy::Id(v) => {
            user = users.filter(id.eq(&v)).get_results::<User>(conn)?;
        }
    }
    if let Some(u) = user.pop() {
        return Ok(u);
    }
    Err(ServiceError::Unauthorized)
}
