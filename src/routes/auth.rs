use actix_web::web::{self, ServiceConfig};

use crate::controllers::auth;

//routes for /auth
pub fn auth_route_config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::resource("/auth")
            .route(web::post().to(auth::login))
            .route(web::delete().to(auth::logout)),
    );
}
