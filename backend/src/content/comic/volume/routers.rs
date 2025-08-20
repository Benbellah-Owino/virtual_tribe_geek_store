use crate::{content::{comic::volume::chapter::routers::chapter_router, VolumeForCreate}, dev_initial::db::Db};
use axum::{
    extract::{Path as AxumPath, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use http::StatusCode;
use serde_json::json;

use super::{
    controllers::{index, store},
};

pub fn volume_router() -> Router<Db> {
    return Router::new()
        .nest("/chapter", chapter_router())
        .route("/:comic", get(list))
        .route("/", post(create));
}

// region:      --- Handlers

/// <h1> Handles creation of Volume </h1>
/// <h2> <b>Endpoint:  <strong>[POST]</strong>  /content/comic/volume/ </b> </h2>
///
/// <h3> Request body</h3>
/// { <br>
///     "synopsis": "A thrilling adventure of unlikely heroes.", <br>
///     "comic": "comic:***", <br>
///     "cover": "path/to/cover.jpg", <br>
///     "no_of_chapters": 16 <br>
/// }<br><br>
///
/// <p>
///     Parameters cannot be empty
/// </p><br><hr>
///
/// <h4>Status Codes</h4>
/// <ul>
///     <li> <b>Ok: Created</b>  : 201</li>
///     <li> <b>Err: Internal Server Error</b> : 500</li>
/// </ul>
#[axum_macros::debug_handler]
async fn create(State(db): State<Db>, Json(payload): Json<VolumeForCreate>) -> impl IntoResponse {
    let db = db.unwrap();
    match store(&db, payload).await {
        Ok(v) => (StatusCode::CREATED, Json(json!({"volume": v}))).into_response(),
        Err(e) => {
            dbg!(e);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}



/// <h1> Handles Getting list of Volume </h1>
/// <h2> <b>Endpoint:  <strong>[GET]</strong>  /content/comic/volume </b> </h2>
///
/// <h3> Request body</h3>
///         NONE<br>
/// 
/// <h3> Response body</h3>
/// [{ <br>
///     "synopsis": "A thrilling adventure of unlikely heroes.", <br>
///     "comic": "comic:***", <br>
///     "cover": "path/to/cover.jpg", <br>
///     "no_of_chapters": 16 <br>
/// }, ...]<br><br>
/// 
/// 
/// <p>
///     Parameters can be empty
/// </p><br><hr>
///
/// <h4>Status Codes</h4>
/// <ul>
///     <li> <b>Ok</b>  : 201</li>
///     <li> <b>Err: Not Found</b> : 404</li>
///     <li> <b>Err: Internal Server Error</b> : 500</li>
/// </ul>
#[axum_macros::debug_handler]
async fn list(State(db): State<Db>, AxumPath(comic): AxumPath<String>) -> impl IntoResponse {
    let db = db.unwrap();
    eprintln!("LIST VOLUMES");
    match index(&db, comic).await {
        Ok(v) => {
            if !v.is_empty() {
                (StatusCode::OK, Json(json!({"volume_list":v}))).into_response()
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
// endregion:   --- Handlers
