pub mod models;
pub mod schema;

use std::env;

use diesel::{r2d2::ConnectionManager, Connection, PgConnection};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn create_pool() -> DbPool {
    
    let database_url = env::var("DATABASE_URL").expect("Missing database URL");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);


    r2d2::Pool::builder()
    .max_size(5)
    .build(manager)
    .expect("Failed to create the connection pool")
}