use std::env;

use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn create_connection_pool() -> Pool {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    //create connection manager for pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    //connection pool
    r2d2::Pool::builder()
        .build(manager)
        .expect("failed to create pool")
}
