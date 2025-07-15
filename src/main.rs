pub mod database;


use std::env;

use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use crate::database::connect_to_db;



#[tokio::main]
async fn main() {

    dotenv().ok();

    let connection = &mut connect_to_db();


    let API_TOKENS: Vec<&str>  = env::var("API_TOKENS").unwrap_or("".to_string()).split_whitespace().collect();

    tracing_subscriber::fmt::init();

    let app = Router::<()>::new()
    .route("/",get(root))
    .merge(admin())
    .merge(fumo());


    println!("Hello, world!");
}


fn admin()-> Router {
    todo!("all the database editing stuff")
}

fn fumo() -> Router {
    todo!("All the reading stuff")
    
}


async fn root() -> &'static str {
    "Welcome to the fumo-API. Learn more at https://github.com/nosesisaid/fumo-api"
}





