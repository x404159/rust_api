use actix_identity::Identity;
use actix_web::{error::BlockingError, web, Error, FromRequest, HttpRequest, HttpResponse};
use diesel::prelude::*;
use futures::future::{err, ok, Ready};
use serde::Deserialize;

use crate::db::Pool;
use crate::utils::verify_hash;
use crate::{errors::ServiceError, models::user::SlimUser};

#[derive(Debug, Deserialize)]
pub struct AuthData {
    email: String,
    password: String,
}

type LoggedUser = SlimUser;

//impl fromrequest so that we can extract LoggedUser from req: HttpRequest as Json
impl FromRequest for LoggedUser {
    type Config = ();
    type Error = Error;
    type Future = Ready<Result<LoggedUser, Error>>;

    fn from_request(req: &HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
        if let Ok(identity) = Identity::from_request(req, payload).into_inner() {
            if let Some(user_json) = identity.identity() {
                if let Ok(user) = serde_json::from_str(&user_json) {
                    return ok(user);
                }
            }
        }
        err(ServiceError::Unauthorized.into())
    }
}

pub async fn logout(id: Identity) -> HttpResponse {
    id.forget();
    HttpResponse::Ok().finish()
}

pub async fn login(
    user_data: web::Json<AuthData>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || query(user_data.into_inner(), pool)).await;

    match res {
        Ok(user) => {
            let user_string = serde_json::to_string(&user).unwrap();
            id.remember(user_string);
            Ok(HttpResponse::Ok().finish())
        }
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}

pub async fn get_me(logged_user: LoggedUser) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({ "email": logged_user.email}))
}

fn query(user_data: AuthData, pool: web::Data<Pool>) -> Result<SlimUser, ServiceError> {
    use crate::models::user::User;
    use crate::schema::users::dsl::{email, users};
    let conn = &pool.get().unwrap();
    let mut items = users
        .filter(email.eq(&user_data.email))
        .load::<User>(conn)?;
    if let Some(user) = items.pop() {
        if let Ok(matching) = verify_hash(&user.password, &user_data.password) {
            if matching {
                return Ok(user.into());
            }
        }
    }
    Err(ServiceError::Unauthorized)
}
