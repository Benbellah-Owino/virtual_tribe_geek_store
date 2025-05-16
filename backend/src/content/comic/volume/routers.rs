use axum::{
    body::Body, extract::{DefaultBodyLimit, Multipart, Path as AxumPath, State}, response::{IntoResponse, Response}, routing::{get, post}, Json, Router
};
use serde::de::value;
use serde_json::json;
use http::StatusCode;
use crate::dev_initial::db::Db;

use super::{controllers::{index, store}, VolumeForCreate};

pub fn volume_router() -> Router<Db>{
    return Router::new()
    .route("/:comic", get(list))
    .route("/", post(create))
}


// region:      --- Handlers
/// <h1> Handles creation of Volume </h1>
/// <h2> <b>Endpoint:  <strong>[POST]</strong>  /comic </b> </h2>
///
/// <h3> Request body</h3>
/// { <br>
///     "writer": ["John Doe, Jane Doe"], <br>
///     "creator": ["creator:***"], <br>
///     "cover" : "path to file"
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
    Json(payload): Json<VolumeForCreate>
) -> impl IntoResponse{
    let db = db.unwrap();
    match store(&db, payload).await{
        Ok(v) => (
            StatusCode::CREATED,
            Json(json!({"volume": v}))
        ).into_response(),
        Err(e) =>{
            dbg!(e);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}


/// <h1> Handles Getting list of Comic </h1>
/// <h2> <b>Endpoint:  <strong>[GET]</strong>  /comic </b> </h2>
///
/// <h3> Request body</h3>
/// NONE
////// <p>
///     Parameters can be empty
/// </p>
///
/// <h4>Status Codes</h4>
/// <ul>
///     <li> <b>Ok</b>  : 201</li>
///     <li> <b>Err</b> : 500</li>
/// </ul>
#[axum_macros::debug_handler]
async fn list(State(db): State<Db>,AxumPath(comic): AxumPath<String>)->impl IntoResponse{
    let db = db.unwrap();

    match index(&db, comic).await{
        Ok(v) => {
            if !v.is_empty(){
                (
                    StatusCode::OK,
                    Json(json!({"volume_list":v}))
                ).into_response()
            }else{
                (StatusCode::NOT_FOUND).into_response()
            }
        }
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}
// endregion:   --- Handlers