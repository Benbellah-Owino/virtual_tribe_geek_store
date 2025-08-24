use crate::content::video::season::controllers::index;
use crate::content::video::season::episode::routers::episode_router;
use crate::{content::video::season::controllers::store, dev_initial::db::Db};
use crate::content::SeasonForCreate;

use axum::routing::{get, post};
use axum::Router;
use axum::{response::IntoResponse, Json,extract::{State, Path as AxumPath}};
use http::StatusCode;
use serde_json::json;
use tracing::debug;


// region:      --- Struct definitions
// endregion:   --- Struct definitions
// region:      --- Router definition
pub fn season_router()-> Router<Db>{
    return Router::new()
        .nest("/episode", episode_router())
        .route("/:video", get(season_list))
        .route("/", post(season_create))
}
// endregion:   --- Router definition
// region:      --- Handlers

async fn season_create(State(db): State<Db>, Json(payload): Json<SeasonForCreate>) -> impl IntoResponse{
    let db = db.unwrap();
    debug!("content/season/season_create -> {:#?}", payload);

    let season = store(&db, payload).await;

    match season {
        Ok(sn) => (StatusCode::CREATED, Json(json!({"season":sn}))).into_response(),
        Err(error) => {
            dbg!(error);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}


async fn season_get(State(db): State<Db>,) -> impl IntoResponse{
    
}

async fn season_list(State(db): State<Db>, AxumPath(video): AxumPath<String>) -> impl IntoResponse{
    let db = db.unwrap();
    eprintln!("LIST SEASONS");
    match index(&db, video).await {
        Ok(seasons) => {
            if !seasons.is_empty() {
                (StatusCode::OK, Json(json!({"season_list":seasons}))).into_response()
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

async fn season_update(State(db): State<Db>) -> impl IntoResponse{}

async fn season_delete(State(db): State<Db>) -> impl IntoResponse{}
// endregion:   --- Handlers
// region:      ---
// endregion:   ---
// region:      ---
// endregion:   ---
// region:      ---
// endregion:   ---