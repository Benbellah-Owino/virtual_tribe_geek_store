use crate::{content::ComicForCreate, dev_initial::db::Db};
use axum::{
    extract::{
        State,
        Path as AxumPath
    },
    response::IntoResponse, routing::Router, Json,
    routing::{get, post, delete}
};
use http::StatusCode;
use serde_json::json;

use super::{controllers::{destroy, index, show, store}, ComicError};

pub fn comic_router() -> Router<Db>{
    Router::new()
        .route("/:comic", get(get_one). delete(delete_comic))
        .route("/",get(list).post(create))
}


// region:      --- Handlers

/// <h1> Handles creation of Comic </h1>
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
    Json(payload): Json<ComicForCreate>
) -> impl IntoResponse{
    let db = db.unwrap();
    let comic = store(&db, payload).await;
    
    match comic{
        Ok(c) => (
            StatusCode::CREATED,
            Json(json!({"comic": c}))
        ).into_response(),

        Err(e) => {
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
async fn list(State(db): State<Db>)->impl IntoResponse{
    let db = db.unwrap();
    let comic_list = index(&db).await;

    match comic_list{
        Ok(c) => {
            if !c.is_empty(){
                (
                    StatusCode::OK,
                    Json(json!({"content_list":c}))
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


/// <h1> Handles Getting one instance of Comic </h1>
/// <h2> <b>Endpoint:  <strong>[GET]</strong>  /comic/:id </b> </h2>
///
/// <h3> Request body</h3>
///     NONE <br>
///
/// <p>
///     Path parameter id represents the Comic ID needed
///     Needs auth token
/// </p>
///
/// <h4>Status Codes</h4>
/// <ul>
///     <li> <b>Ok</b>  : 201</li>
///     <li> <b>Err</b> : 500</li>
/// </ul>
#[axum_macros::debug_handler]
async fn get_one(State(db): State<Db>, AxumPath(comic_id): AxumPath<String>)->impl IntoResponse{
    let db = db.unwrap();

    match show(&db, comic_id).await{
        Ok(c) =>{
            (
                StatusCode::OK,
                Json(json!({"comic": c}))
            ).into_response()
        },
        Err(e) =>{
            match e{
                ComicError::NotFound => (StatusCode::NOT_FOUND).into_response(),
                _=> (StatusCode::INTERNAL_SERVER_ERROR).into_response()
            }
        }
    }
}


/// <h1> Handles creation of Comic </h1>
/// <h2> <b>Endpoint:  <strong>[PATCH]</strong>  /comic/id </b> </h2>
///
/// <h3> Request body</h3>
/// { <br>
///     field: "writers",
///     value: ["Mark", "Henry"]
/// }<br><br>
///
/// <p>
///     Path parameter is the comic id
///     Needs auth token
/// </p>
///
/// <h4>Status Codes</h4>
/// <ul>
///     <li> <b>Ok</b>  : 201</li>
///     <li> <b>Err</b> : 500</li>
/// </ul>
#[axum_macros::debug_handler]
async fn edit(
    State(db): State<Db>,
    Json(payload): Json<ComicForCreate> //Create ComicForUpdate
) -> impl IntoResponse{
    //TODO: Implement edit functionalities for comics
    //TODO: Make sure only creators and admin can edit comic
}



/// <h1> Deletes an instance of Comic </h1>
/// <h2> <b>Endpoint:  <strong>[DELETE]</strong>  /comic/:id </b> </h2>
///
/// <h3> Request body</h3>
///     NONE <br>
///
/// <p>
///     Path parameter id represents the Comic ID needed
///     Needs auth token
/// </p>
///
/// <h4>Status Codes</h4>
/// <ul>
///     <li> <b>Ok</b>  : 201</li>
///     <li> <b>Err</b> : 500</li>
/// </ul>
#[axum_macros::debug_handler]
async fn delete_comic(State(db): State<Db>, AxumPath(comic_id): AxumPath<String>)->impl IntoResponse{
    let db = db.unwrap();

    match destroy(&db, comic_id).await{
        Ok(c) => (
            StatusCode::OK,
            Json(json!({"comic":c}))
        ).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR
        ).into_response()
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