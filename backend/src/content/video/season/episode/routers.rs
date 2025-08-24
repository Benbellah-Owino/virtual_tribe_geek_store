// region:      --- Imports
use axum::{extract::{Path as AxumPath, State}, response::IntoResponse, routing::{get, post}, Json, Router};
use http::StatusCode;
use serde_json::json;
use tracing::debug;

use crate::{content::{video::season::episode::controllers::{index, store}, EpisodeForCreate}, dev_initial::db::Db};


// endregion:   --- Imports


// region:      --- Router
pub fn episode_router() -> Router<Db>{
    Router::new()
        .route("/:season", get(episode_list))
        .route("/", post(episode_create))
}
// endregion:   --- Router


// region:      --- Router handlers
async fn episode_create(State(db): State<Db>, Json(payload): Json<EpisodeForCreate>) -> impl IntoResponse{
    let db = db.unwrap();
    debug!("content/episode/episode_create -> {:#?}", payload);

    match store(&db, payload).await{
        Ok(episode) => (StatusCode::CREATED, Json(json!({"episode":episode}))).into_response(),
        Err(error) =>{
            dbg!(error);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}


async fn episode_get(State(db): State<Db>) -> impl IntoResponse{}
async fn episode_list(State(db): State<Db>, AxumPath(season): AxumPath<String>) -> impl IntoResponse{ 
    let db = db.unwrap();
    eprintln!("LIST EPISODES");
    match index(&db, season).await {
        Ok(episodes) => {
            if !episodes.is_empty() {
                (StatusCode::OK, Json(json!({"episode_list":episodes}))).into_response()
            } else {
                (StatusCode::NOT_FOUND).into_response()
            }
        }
        Err(e) => {
            eprintln!("{:#?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}
async fn episode_upate(State(db): State<Db>) -> impl IntoResponse{}
async fn episode_delete(State(db): State<Db>) -> impl IntoResponse{}
// endregion:   --- Router handlers
// region:      ---
// endregion:   ---