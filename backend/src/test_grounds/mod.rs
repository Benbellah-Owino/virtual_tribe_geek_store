use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use http::StatusCode;

use crate::{content::comic::volume::chapter::ChapterForCreate, dev_initial::db::Db, helpers::db::id_from_thing};

pub fn test_grounds_router() -> Router<Db>{
    return Router::new().route("/test_api", post(test_fn))
}

async fn test_fn(
    State(db) : State<Db>,
    Json(payload): Json<ChapterForCreate>
) -> impl IntoResponse {
    let id = id_from_thing(payload.volume);
    eprintln!("Id -> {id}");
    return (StatusCode::OK, id)
}