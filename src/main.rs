use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpServer};

use server::{
    routes::{auth, not_found, user, users},
    utils,
};

#[cfg(test)]
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = "127.0.0.1:8000";
    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();
    let conn_pool = server::db::create_connection_pool();
    let domain = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_owned());
    HttpServer::new(move || {
        App::new()
            .data(conn_pool.clone())
            //enable logger middleware
            .wrap(middleware::Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(utils::SECRET_KEY.as_bytes())
                    .name("auth")
                    .path("/")
                    .domain(domain.as_str())
                    .max_age(86400)
                    .secure(false),
            ))
            //limit the maximum amount of data that server will except
            .data(web::JsonConfig::default().limit(4096))
            .configure(users::users_route_config)
            .configure(user::user_route_config)
            .configure(auth::auth_route_config)
            .default_service(web::route().to(not_found::handle_404))
    })
    .bind(address)?
    .run()
    .await
}
