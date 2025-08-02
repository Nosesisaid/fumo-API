use std::{env, net::SocketAddr};

use axum::{
    Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, patch, post},
};
use axum_extra::TypedHeader;
use dotenvy::dotenv;
use fumo_db::{DbPool, create_pool, models::NewFumo};
use headers::{Authorization, authorization::Bearer};
use serde::Deserialize;

mod admin;

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
        .with_state(api_state);

    println!("Hello, world!. Trying to listen on {port}");

    let address = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn admin() -> Router<AppState> {
    Router::new()
        .route("/admin/fumos/new", post(add_fumo))
        .route("/admin/fumo/{fumo}/involved", patch(update_involved_fumos))
}

async fn add_fumo(
    authorization_header: Option<TypedHeader<Authorization<Bearer>>>,
    State(state): State<AppState>,
    Json(payload): Json<NewFumo>,
) -> impl IntoResponse {
    if !admin::is_valid_api_token(authorization_header, &state.api_tokens) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let Ok(mut conn) = state.db.get() else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    let result = fumo_db::operations::add_fumo(&mut conn, payload);

    match result {
        Ok(f) => Ok(Json(f)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn update_involved_fumos(
    authorization_header: Option<TypedHeader<Authorization<Bearer>>>,
    Path(fumo): Path<String>,
    State(state): State<AppState>,
    Json(payload): Json<Vec<String>>,
) -> impl IntoResponse {
    if !admin::is_valid_api_token(authorization_header, &state.api_tokens) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let Ok(mut conn) = state.db.get() else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    let Ok(fumo) = fumo.parse() else {
        return Err(StatusCode::BAD_REQUEST);
    };

    let res = fumo_db::operations::edit_involved(&mut conn, fumo, payload);

    match res {
        Ok(f) => Ok(Json(f)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
fn fumo() -> Router<AppState> {
    //todo!("All the reading stuff")
    Router::new()
        .route("/fumos/list_all", get(list_all))
        .route("/fumos", get(list))
        .route("/fumos/count", get(all_fumo_count))
        .route("/fumos/{fumo}/count", get(fumo_count))
}

//Test function. The DB never should return everything unpaginated
async fn list_all(
    authorization_header: Option<TypedHeader<Authorization<Bearer>>>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if !admin::is_valid_api_token(authorization_header, &state.api_tokens) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let Ok(mut conn) = state.db.get() else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    let results = fumo_db::operations::fetch_fumos(&mut conn, 0, None);
    if let Ok(res) = results {
        Ok(Json(res))
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct Pagination {
    page: Option<u32>,
    limit: Option<u32>,
}

async fn list(
    Query(pagination): Query<Pagination>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let Ok(mut conn) = state.db.get() else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };
    let page = pagination.page.unwrap_or(1);
    let limit = pagination.limit.unwrap_or(15);

    let offset = (limit * page) - limit;

    let results = fumo_db::operations::fetch_fumos(&mut conn, offset.into(), Some(limit.into()));

    match results {
        Ok(r) => Ok(Json(r)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn fumo_count(Path(fumo): Path<String>, State(state): State<AppState>) -> impl IntoResponse {
    let Ok(mut conn) = state.db.get() else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    let count = fumo_db::operations::fumo_count_by(&mut conn, fumo);

    match count {
        Ok(c) => Ok(Json(c)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn all_fumo_count(State(state): State<AppState>) -> impl IntoResponse {
    let Ok(mut conn) = state.db.get() else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    let count = fumo_db::operations::fumo_count(&mut conn);

    match count {
        Ok(c) => Ok(Json(c)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn root() -> &'static str {
    "Welcome to the fumo-API. Learn more at https://github.com/nosesisaid/fumo-api"
}
