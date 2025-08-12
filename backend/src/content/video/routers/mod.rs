// region:      --- Imports
use crate::{content::{video::controllers::store, VideoForCreate}, dev_initial::db::Db};
use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use http::StatusCode;
use serde_json::json;
use tracing::debug;
// endregion:   --- Imports

// region:      --- Struct Definitions 

// endregion:   --- Struct Definitions 


// region:      --- Router 
pub fn video_router() -> Router<Db> {
    Router::new()
        .route("/", post(video_create))
}
// endregion:   --- Router

// region:      --- Handlers

#[axum_macros::debug_handler]
async fn video_create(State(db): State<Db>, Json(payload):Json<VideoForCreate>)-> impl IntoResponse{
    let db = db.unwrap();
    debug!("content/video/video_create -> {:#?}", payload);
    
    let video = store(&db, payload).await;

    match video{
        Ok(vid) => (StatusCode::CREATED, Json(json!({"video":vid}))).into_response(),
        Err(error) =>{
            dbg!(error);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}
async fn video_list(){}
async fn video_get(){}
async fn video_update(){}
async fn video_delete(){}
// endregion:   --- Handlers
// region:      ---
// endregion:   ---
// region:      ---
// endregion:   ---
// region:      ---
// endregion:   ---