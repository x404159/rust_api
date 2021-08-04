use actix_identity::Identity;
use actix_web::{web, HttpResponse};

use crate::db::Pool;
use crate::models::{dbmethods::login_user, user::AuthData};
use crate::{errors::ServiceError, models::user::SlimUser};

//route handles
//DELETE /auth
pub async fn logout(id: Identity) -> HttpResponse {
    id.forget();
    HttpResponse::Ok().finish()
}

//POST /auth
pub async fn login(
    user_data: web::Json<AuthData>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let user = login_user(user_data.into_inner(), pool)?;

    let user_string = serde_json::to_string(&user).unwrap();
    id.remember(user_string);
    Ok(HttpResponse::Ok().finish())
}

//GET /auth
pub async fn get_me(logged_user: SlimUser) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({ "email": logged_user.email}))
}
