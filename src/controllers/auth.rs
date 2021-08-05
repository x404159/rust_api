use actix_web::http::header;
use actix_web::{web, HttpResponse};

use crate::db::db::Pool;
use crate::errors::ServiceError;
use crate::models::{dbmethods::login_user, user::AuthData};
use crate::utils;

//route handles
//DELETE /auth
pub async fn logout() -> HttpResponse {
    HttpResponse::Ok()
        .set_header(header::AUTHORIZATION, "")
        .finish()
}

//POST /auth
pub async fn login(
    user_data: web::Json<AuthData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let user = login_user(user_data.into_inner(), pool)?;
    let token = utils::create_jwt(user)?;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "token": token })))
}
