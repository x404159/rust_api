use crate::controllers::user;
use actix_web::web::{self, ServiceConfig};

//routes for /user
pub fn user_route_config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::resource("/user")
            .route(web::get().to(user::get_me))
            .route(web::patch().to(user::update_user))
            .route(web::delete().to(user::remove_account)),
    )
    .route("/user/{id}", web::get().to(user::get_user_by_id))
    .route("/testing", web::get().to(user::test_route));
}
