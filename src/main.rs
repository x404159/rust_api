use actix_web::{middleware, web, App, HttpServer};

use server::{
    middlewares,
    routes::{auth, not_found, user, users},
};

#[cfg(test)]
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = "0.0.0.0:8000";
    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();
    let conn_pool = server::db::create_connection_pool();
    HttpServer::new(move || {
        App::new()
            .data(conn_pool.clone())
            //enable logger middleware
            .wrap(middleware::Logger::default())
            .wrap(middlewares::auth::Auth)
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
