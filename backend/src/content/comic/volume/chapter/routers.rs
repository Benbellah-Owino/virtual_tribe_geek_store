use crate::{
    content::comic::volume::chapter::{
        controllers::{index, show, store, update},
        ChapterForCreate,
    },
    dev_initial::db::Db, file_upload::{small_file::{self, extract_image}, storage::{self, save_to_disk}},
};
use crate::content::ContentForUpdate;
use axum::{
    extract::{Path as AxumPath, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use axum::extract::Multipart;
use http::StatusCode;
use serde::Deserialize;
use serde_json::json;
use tracing::debug;

pub fn chapter_router() -> Router<Db> {
    return Router::new()

        .route("/file/upload/:id", post(file_upload))
        .route("/:volume", get(list))
        .route("/", post(create));
}

// region:      --- Query Structs
#[derive(Deserialize, Clone, Debug)]
pub struct GetChapterQuery {
    pub volume: Option<String>,
    pub chapter: Option<String>,
}
// endregion:   --- Query Structs
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
async fn create(State(db): State<Db>, Json(payload): Json<ChapterForCreate>) -> impl IntoResponse {
    let db = db.unwrap();
    match store(&db, payload).await {
        Ok(chapter) => (StatusCode::CREATED, Json(json!({"chapter": chapter}))).into_response(),
        Err(error) => {
            dbg!(error);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}

#[axum::debug_handler]
async fn file_upload(
    State(db): State<Db>,
    AxumPath(id): AxumPath<String>,
    multipart: Multipart,
) -> impl IntoResponse {
    println!("Uploading");
    let db = db.unwrap();

    let id_string = id.split(":").collect::<Vec<&str>>();
    // Attempt to extract the image file from multipart
    let file = match extract_image(multipart).await {
        Ok(file) => file,
        Err(e) => {
            println!("File extraction failed.");
            match e {
                small_file::Error::TooBig => {
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(json!({"msg": "File is too big, use a smaller image"})),
                    )
                }
                _ => {
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(json!({"msg": "Update Failed"})),
                    )
                }
            }
        }
    };

    // Store the file in the specified director
    dbg!(&id_string[1]);
    let path = format!("media\\comics\\{}", id_string[1]);
    debug!("content_path-> {path}");
    let file = match storage::store(Some(path), format!("{}comic", id_string[1]), file).await {
        Some(f) => f,
        None => {
            println!("File storage failed.");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"msg": "File storage error"})),
            );
        }
    };

    // Check if we have the ID in request extensions
    // Proceed with saving to disk and updating details
    let file = (&file.0.clone(), file.1, file.2);
    let path = file.0.clone();
    let _ = save_to_disk(file).await;
    let value = path.to_str().unwrap();
    debug!("{value}");

    let payload = ContentForUpdate {
        field: "file".to_string(),
        value: value.to_string(),
    };

    println!("{:?}", &path);

    match update(&db, id_string[1], payload).await {
        Ok(_) => (StatusCode::OK, Json(json!({"msg": "File uploaded"}))),
        Err(e) => {
            dbg!(e);
            (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"msg": "Upload error"})),
        )
    },
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
async fn list(State(db): State<Db>, AxumPath(volume): AxumPath<String>) -> impl IntoResponse {
    let db = db.unwrap();
    eprintln!("LIST CHAPTERS");
    match index(&db, volume).await {
        Ok(chapters) => {
            if !chapters.is_empty() {
                (StatusCode::OK, Json(json!({"chapters_list":chapters}))).into_response()
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
// async fn get_one(State(db): State<Db>, AxumPath(comic_id): AxumPath<String>)->impl IntoResponse{
#[axum_macros::debug_handler]
async fn get_chapters_for_volume(
    State(db): State<Db>,
   // Query(chapter_query): Query<GetChapterQuery>,
) -> impl IntoResponse {
    // let db = db.unwrap();


    // match show(&db, chapter_query).await{
    //     Ok(chapter) =>{}
    // }
    unimplemented!()
}