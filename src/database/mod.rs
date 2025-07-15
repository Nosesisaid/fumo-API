pub mod models;
pub mod schema;

use std::env;

use diesel::{Connection, PgConnection};

pub fn connect_to_db() -> PgConnection {
    
    let database_url = env::var("DATABASE_URL").expect("Missing database URL");
    PgConnection::establish(&database_url).unwrap_or_else(|err| panic!("Error connecting to databse {} {}", database_url, err))
}