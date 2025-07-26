use diesel::{prelude::*};
use dotenvy::dotenv;
use std::env;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


pub mod schema;
pub mod models;
pub mod operations;

use diesel::{r2d2::ConnectionManager, PgConnection};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn create_pool(database_url: String) -> DbPool {
    
    let manager = ConnectionManager::<PgConnection>::new(&database_url);


    r2d2::Pool::builder()
    .max_size(5)
    .build(manager)
    .expect("Failed to create the connection pool")
}