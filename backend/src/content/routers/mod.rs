use axum::{
    extract::State,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use http::StatusCode;
use serde_json::json;
use tracing::{debug, info};

use crate::{content::ContentForCreateServer, dev_initial::db::Db};

use super::{
    controllers::{get_all, store},
    genre::routers::genre_router,
};
use crate::content::ContentForCreateClient;

// region:      --- Router
pub fn content_router() -> Router<Db> {
    Router::new()
        .nest("/genre", genre_router())
        .route("/", get(list).post(create))
}
// endregion:   --- Router

// region:      --- Handlers

/// <h1> Handles registration of Content </h1>
/// <h2> <b>Endpoint:  <strong>[POST]</strong>  /content </b> </h2>
///
/// <h3> Request body</h3>
/// { <br>
///     "title": "test_creator1", <br>
///     "studio": "studio:sxshus8430sdd9dde"
///     "description": "Nice content", <br>
///     "audiences": "FAMILY", <br>
///     "recom_price": "password" <br>
/// }<br><br>
///
/// <p>
///     Parameters cannot be empty
/// </p>
///
/// <h4>Status Codes</h4>
/// <ul>
///     <li> <b>Ok</b>  : 201</li>
///     <li> <b>Err</b> : 500</li>
/// </ul>

#[axum_macros::debug_handler]
async fn create(
    State(db): State<Db>,
    Json(payload): Json<ContentForCreateClient>,
) -> impl IntoResponse {
    let db = db.unwrap();
    dbg!(&payload);
    let content: ContentForCreateServer = payload.into();
    dbg!(&content);
    let content = store(content, &db).await;
    eprintln!("{:?}", content);
    match content {
        Ok(c) => (
            StatusCode::CREATED,
            Json(json!({
                "content": c
            })),
        )
            .into_response(),
        Err(e) => {
            dbg!(e);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}

async fn list(State(db): State<Db>) -> impl IntoResponse {
    let db = db.unwrap();
    info!("list");
    let content_list = get_all(&db).await;

    match content_list {
        Ok(c) => {
            debug!("{:?}", c);
            if !c.is_empty() {
                (
                    StatusCode::OK,
                    Json(json!({
                        "content_list": c
                    })),
                )
                    .into_response()
            } else {
                (StatusCode::NOT_FOUND).into_response()
            }
        }
        Err(e) => {
            debug!("{:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}

#[allow(dead_code)]
async fn show(State(_db): State<Db>) -> impl IntoResponse {}

#[allow(dead_code)]
async fn edit(State(_db): State<Db>) -> impl IntoResponse {}

#[allow(dead_code)]
async fn delete_content(State(_db): State<Db>) -> impl IntoResponse {}
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
