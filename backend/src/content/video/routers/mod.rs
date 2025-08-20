// region:      --- Imports
use crate::{
    content::{
        video::{
            controllers::{index, show, store}, season::routers::season_router, VideoError
        },
        VideoForCreate,
    },
    dev_initial::db::Db,
};
use axum::{
    extract::{Query, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use http::StatusCode;
use serde::Deserialize;
use serde_json::json;
use tracing::debug;
// endregion:   --- Imports

// region:      --- Struct Definitions
#[derive(Deserialize, Clone, Debug)]
pub struct GetVideoQuery {
    pub video: Option<String>,
    pub content: Option<String>,
}
// endregion:   --- Struct Definitions

// region:      --- Router
pub fn video_router() -> Router<Db> {
    Router::new()
        .nest("/season", season_router())
        .route("/index", get(video_get))
        .route("/", post(video_create).get(video_list))
}
// endregion:   --- Router

// region:      --- Handlers

#[axum_macros::debug_handler]
async fn video_create(
    State(db): State<Db>,
    Json(payload): Json<VideoForCreate>,
) -> impl IntoResponse {
    let db = db.unwrap();
    debug!("content/video/video_create -> {:#?}", payload);

    let video = store(&db, payload).await;

    match video {
        Ok(vid) => (StatusCode::CREATED, Json(json!({"video":vid}))).into_response(),
        Err(error) => {
            dbg!(error);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}
async fn video_list(State(db):State<Db>) -> impl IntoResponse {
    let db = db.unwrap();
    let video_list = index(&db).await;
    println!("video list");
    match video_list {
        Ok(videos) => {
            if !videos.is_empty() {
                (StatusCode::OK, Json(json!({"video_list":videos}))).into_response()
            } else {
                eprintln!("Empty");
                (StatusCode::NOT_FOUND).into_response()
            }
        }
        Err(e) => {
            eprintln!("ERROR: {:#?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}

#[axum_macros::debug_handler]
async fn video_get(
    State(db): State<Db>,
    Query(video_query): Query<GetVideoQuery>,
) -> impl IntoResponse {
    let db = db.unwrap();

    match show(&db, video_query).await {
        Ok(video) => {
            debug!("{:#?}", video);
            (StatusCode::OK, Json(json!({"video": video}))).into_response()
        }
        Err(e) => match e {
            VideoError::NotFound => (StatusCode::NOT_FOUND).into_response(),
            _ => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        },
    }
}
async fn video_update() {}
async fn video_delete() {}
// endregion:   --- Handlers
// region:      ---
// endregion:   ---
// region:      ---
// endregion:   ---
// region:      ---
// endregion:   ---
