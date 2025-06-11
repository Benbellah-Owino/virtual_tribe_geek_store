use axum::{extract::{Path as AxumPath, State}, response::IntoResponse, routing::{get, post}, Json, Router};
use http::StatusCode;
use serde_json::json;
use crate::{content::comic::volume::{self, chapter::{controllers::{index, store}, ChapterForCreate}}, dev_initial::db::Db};


pub fn chapter_router() -> Router<Db>{
    return Router::new()
        .route("/:volume", get(list))
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
    State(db) : State<Db>,
    Json(payload): Json<ChapterForCreate>
) -> impl IntoResponse{
    let db = db.unwrap();
    match store(&db, payload).await{
        Ok(chapter) => (
            StatusCode::CREATED,
            Json(json!({"chapter": chapter}))
        ).into_response(),
        Err(error) =>{
            dbg!(error);
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
async fn list(State(db): State<Db>,AxumPath(volume): AxumPath<String>)->impl IntoResponse{
    let db = db.unwrap();
    eprintln!("LIST CHAPTERS");
    match index(&db, volume).await{
        Ok(chapters) => {
            if !chapters.is_empty(){
                (
                    StatusCode::OK,
                    Json(json!({"chapters_list":chapters}))
                ).into_response()
            }else{
                (StatusCode::NOT_FOUND).into_response()
            }
        }
        Err(e) => {
            eprintln!("{:#?}",e);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}
