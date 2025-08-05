use std::{env, net::SocketAddr};

use axum::{http::{HeaderValue, Method}, routing::get, Router};
use dotenvy::dotenv;
use fumo_db::{DbPool, create_pool};
use tower_http::cors::CorsLayer;

use crate::{admin_router::admin, fumos_router::fumo};

mod admin_router;
mod fumos_router;
mod util;

#[derive(Clone)]
struct AppState {
    db: DbPool,
    api_tokens: Vec<String>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let api_tokens: Vec<String> = env::var("API_TOKENS")
        .unwrap_or("".to_string())
        .split_whitespace()
        .map(|s| s.into())
        .collect();
    let port: u16 = env::var("FUMO_API_PORT")
        .unwrap_or("3000".to_string())
        .parse()
        .unwrap();

    let db_pool = create_pool(database_url);

    let api_state = AppState {
        db: db_pool,
        api_tokens,
    };

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        // .merge(admin())
        .merge(fumo())
        .merge(admin())
        .layer(CorsLayer::new().allow_origin("*".parse::<HeaderValue>().unwrap()).allow_methods([Method::GET]))
        .with_state(api_state);

    println!("Hello, world!. Trying to listen on {port}");

    let address = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Welcome to the fumo-API. Learn more at https://github.com/nosesisaid/fumo-api"
}
