use axum::{extract::{Path, Query, State}, http::StatusCode, response::IntoResponse, Json, Router};
use axum_extra::TypedHeader;
use fumo_db::models::is_valid_involvable;
use headers::{authorization::Bearer, Authorization};
use serde::Deserialize;
use axum::routing::method_routing::get;
use crate::{util, AppState};


pub fn fumo() -> Router<AppState> {
    //todo!("All the reading stuff")
    Router::new()
        .route("/fumos/list_all", get(list_all))
        .route("/fumos/random", get(random_fumo))
        .route("/fumos/{fumo}/random", get(random_specific_fumo))
        .route("/fumos", get(list))
        .route("/fumos/count", get(all_fumo_count))
        .route("/fumos/{fumo}/count", get(fumo_count))
        
}

//Test function. The DB never should return everything unpaginated
async fn list_all(
    authorization_header: Option<TypedHeader<Authorization<Bearer>>>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if !util::is_valid_api_token(authorization_header, &state.api_tokens) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let Ok(mut conn) = state.db.get() else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    let results = fumo_db::operations::fetch_fumos(&mut conn, 0, None,false);
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

    let results = fumo_db::operations::fetch_fumos(&mut conn, offset.into(), Some(limit.into()),false);

    match results {
        Ok(r) => Ok(Json(r)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn fumo_count(Path(fumo): Path<String>, State(state): State<AppState>) -> impl IntoResponse {
    let Ok(mut conn) = state.db.get() else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    let count = fumo_db::operations::fumo_count_by(&mut conn, fumo, false);

    match count {
        Ok(c) => Ok(Json(c)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn all_fumo_count(State(state): State<AppState>) -> impl IntoResponse {
    let Ok(mut conn) = state.db.get() else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    let count = fumo_db::operations::fumo_count(&mut conn,false);

    match count {
        Ok(c) => Ok(Json(c)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}


async fn random_specific_fumo(Path(fumo): Path<String>,State(state): State<AppState>) -> impl IntoResponse {
        let Ok(mut conn) = state.db.get() else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    if !is_valid_involvable(&fumo){
        return Err(StatusCode::NOT_FOUND);
    }

    match fumo_db::operations::get_random(&mut conn, Some(fumo), false){
        Ok(c) => Ok(Json(c)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}


async fn random_fumo(State(state): State<AppState>) -> impl IntoResponse {
        let Ok(mut conn) = state.db.get() else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    match fumo_db::operations::get_random(&mut conn, None, false){
        Ok(c) => Ok(Json(c)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}