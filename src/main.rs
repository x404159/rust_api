use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpServer};

use server::{
    routes::{auth, not_found, user, users},
    utils,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = "127.0.0.1:8000";
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
            .service(
                web::resource("/users")
                    .route(web::post().to(users::post_user))
                    .route(web::get().to(users::get_users)),
            )
            .route("/users/{id}", web::patch().to(users::change_account_type))
            .service(
                web::resource("/user")
                    .route(web::get().to(user::get_me))
                    .route(web::patch().to(user::update_user))
                    .route(web::delete().to(user::remove_account)),
            )
            .route("user/{id}", web::get().to(user::get_user_by_id))
            .service(
                web::resource("/auth")
                    .route(web::get().to(auth::get_me))
                    .route(web::post().to(auth::login))
                    .route(web::delete().to(auth::logout)),
            )
            .default_service(web::route().to(not_found::handle_404))
    })
    .bind(address)?
    .run()
    .await
}
