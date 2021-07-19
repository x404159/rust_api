use actix_identity::Identity;
use actix_web::error::BlockingError;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;

use crate::db::Pool;
use crate::errors::ServiceError;
use crate::models::user::{SlimUser, User, UserChange};
use crate::utils::{get_logged_user, hash_password};

pub enum FindBy {
    Email(String),
    Id(i64),
}

pub async fn get_me(id: Identity, pool: web::Data<Pool>) -> Result<HttpResponse, ServiceError> {
    let user = get_logged_user(&id)?;
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
    let id = match id.into_inner().parse::<i64>() {
        Ok(v) => v,
        Err(_) => return Err(ServiceError::BadRequest("invalid id".to_owned())),
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

pub async fn update_user(
    id: Identity,
    updates: web::Json<UserChange>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let user = get_logged_user(&id)?;
    let updates = updates.into_inner();
    if updates.password.is_some() {
        //logging out since attempting to change password
        id.forget();
    }
    let res = web::block(move || user_update(user, updates, pool)).await;

    match res {
        Ok(changed) => Ok(HttpResponse::Ok().json(changed)),
        Err(e) => match e {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}

pub async fn remove_account(
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let user = get_logged_user(&id)?;
    let res = web::block(move || delete_account(user.email, pool)).await;

    match res {
        Ok(b) => {
            if b {
                id.forget();
                Ok(HttpResponse::Ok()
                    .json(serde_json::json!({ "msg": "account deleted successfully" })))
            } else {
                Ok(HttpResponse::Ok()
                    .json(serde_json::json!({ "msg": "could not delete account" })))
            }
        }
        Err(e) => match e {
            BlockingError::Error(service_err) => Err(service_err),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}

fn delete_account(user_email: String, pool: web::Data<Pool>) -> Result<bool, ServiceError> {
    use crate::schema::users::dsl::{email, users};
    let conn = &pool.get().unwrap();
    let result = diesel::delete(users)
        .filter(email.eq_all(user_email))
        .execute(conn)?;
    if result > 0 {
        Ok(true)
    } else {
        Ok(false)
    }
}

fn user_update(
    user: SlimUser,
    updates: UserChange,
    pool: web::Data<Pool>,
) -> Result<UserChange, ServiceError> {
    use crate::schema::users::dsl::{email, users};
    let mut updates = updates;
    if let Some(ref mut passwd) = updates.password {
        *passwd = hash_password(&passwd)?;
    }
    dbg!(&updates);
    let conn = &pool.get().unwrap();
    let result = diesel::update(users)
        .filter(email.eq(user.email))
        .set(&updates)
        .get_result::<UserChange>(conn)?;
    Ok(result)
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
