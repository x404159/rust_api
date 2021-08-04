use crate::controllers::users;

use actix_web::web::{self, ServiceConfig};

//routes
pub fn users_route_config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .route(web::post().to(users::post_user))
            .route(web::get().to(users::get_users)),
    )
    .route("/users/{id}", web::patch().to(users::change_account_type));
}
