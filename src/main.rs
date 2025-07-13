use std::env;

use serde::{Deserialize, Serialize};
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

const API_TOKENS: Vec<String>  = env::var("API_TOKENS").unwrap().split(" ").collect::<Vec<String>>();


#[tokio::main]
async fn main() {

    tracing_subscriber::fmt::init();

    let app = Router::new()
    .route("/",get(root));

    println!("Hello, world!");
}



async fn root() -> &'static str {
    "Welcome to the fumo-API. Learn more at https://github.com/nosesisaid/fumo-api"
}


#[derive(Deserialize)]
struct Fumo {
    id: u64,
    caption: String,
    url: String,
    fumos: Vec<String>
}