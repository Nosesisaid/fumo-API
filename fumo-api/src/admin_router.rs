
use axum::{extract::{Path, Query, State}, http::StatusCode, response::IntoResponse, Json, Router};
use axum_extra::TypedHeader;
use fumo_db::models::{is_valid_involvable, NewFumo};
use headers::{authorization::Bearer, Authorization};
use serde::Deserialize;
use axum::routing::method_routing::{post,patch};
use crate::{util, AppState};

pub fn admin() -> Router<AppState> {
    Router::new()
        .route("/admin/fumos/new", post(add_fumo))
        .route("/admin/fumo/{fumo}/involved", patch(update_involved_fumos))
}

async fn add_fumo(
    authorization_header: Option<TypedHeader<Authorization<Bearer>>>,
    State(state): State<AppState>,
    Json(payload): Json<NewFumo>,
) -> impl IntoResponse {
    if !util::is_valid_api_token(authorization_header, &state.api_tokens) {
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
    if !util::is_valid_api_token(authorization_header, &state.api_tokens) {
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