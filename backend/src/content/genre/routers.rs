// region:      --- Imports
use axum::{
    extract::State,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use http::StatusCode;
use serde_json::json;

use crate::dev_initial::db::Db;

use super::{
    controllers::{list, store},
    GenreForCreate,
};
// endregion:   --- Imports

// region:      --- RouterFunction
pub fn genre_router() -> Router<Db> {
    return Router::new().route("/", get(list_handler).post(create_handler));
}
// endregion:   --- RouterFunction

// region:      --- Handlers

//TODO: Make this admin only function
pub async fn create_handler(
    State(db): State<Db>,
    Json(payload): Json<GenreForCreate>,
) -> impl IntoResponse {
    let db = db.unwrap();
    let new_genre = store(payload, &db).await;

    match new_genre {
        Ok(g) => (StatusCode::CREATED, Json(json!({"genre": g}))).into_response(),
        Err(e) => {
            dbg!(e);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}

pub async fn list_handler(State(db): State<Db>) -> impl IntoResponse {
    let db = db.unwrap();
    let genre_list = list(&db).await;

    match genre_list {
        Ok(g) => (StatusCode::OK, Json(json!({"genres": g}))).into_response(),
        Err(e) => {
            dbg!(e);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}
// endregion:   --- Handlers
// region:      ---
// endregion:   ---

// region:      ---
// endregion:   ---
// region:      ---
// endregion:   ---
// region:      ---
// endregion:   ---
// region:      ---
// endregion:   ---
