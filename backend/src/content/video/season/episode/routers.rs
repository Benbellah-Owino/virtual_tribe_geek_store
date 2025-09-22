// region:      --- Imports
use axum::{body::Body,extract::{Multipart, Path as AxumPath, State}, response::IntoResponse, routing::{get, post}, Json, Router};
use http::{HeaderMap, StatusCode};
use serde_json::json;
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use tracing::debug;
use hyper::header;

use crate::{content::{video::season::episode::{controllers::{index, show, store, update}, routers::file_handlers::convert_to_mp4}, ContentForUpdate, EpisodeForCreate}, dev_initial::db::Db, file_upload::{small_file::{self, extract_image}, storage::{self, save_to_disk}}};

mod file_handlers;
// endregion:   --- Imports


// region:      --- Router
pub fn episode_router() -> Router<Db>{
    Router::new()
        .route("/file/*file_path", get(video_stream))
        .route("/show/:chapter", get(episode_get))
        .route("/video/upload/:id", post(video_upload)) 
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


async fn episode_get(State(db): State<Db>, AxumPath(episode): AxumPath<String>) -> impl IntoResponse{ 
    let db = db.unwrap();
    eprintln!("GET EPISODE");

    match show(&db, episode).await{
        Ok(episode) => (StatusCode::OK, Json(json!({"episode": episode}))),
        Err(_e) => (
            StatusCode::NOT_FOUND,
            Json(json!({"msg": "Episode is not found"})),
        ),

    }
}

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

/// <h1> Handles the uploading of the actual comic file </h1>
/// <h2> <b>Endpoint:  <strong>[POST]</strong>  /content/comic/volume/chapter/file/upload/:id </b> </h2>
///
/// <h3> Request body</h3>
/// { <br>
///     "FILE DATA"
/// }<br><br>
///
/// <p>
///      Path parameter id represents the id of Chapter whose file is being uploaded
/// </p><br><hr>
///
/// <h4>Status Codes</h4>
/// <ul>
///     <li> <b>Ok: Created</b>  : 201</li>
///     <li> <b>Err: Bad Request</b> : 400</li>
///     <li> <b>Err: Internal Server Error</b> : 500</li>
/// </ul>
#[axum::debug_handler]
async fn video_upload1(
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
    let path = format!("media\\video\\files\\vids\\{}", id_string[1]);
    debug!("content_path-> {path}");
    let file = match storage::store(Some(path), format!("{}video", id_string[1]), file).await {
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
        Ok(_) => (StatusCode::CREATED, Json(json!({"msg": "File uploaded"}))),
        Err(e) => {
            dbg!(e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"msg": "Upload error"})),
            )
        }
    }
}

#[axum::debug_handler]
async fn video_upload(
    State(db): State<Db>,
    AxumPath(id): AxumPath<String>,
    multipart: Multipart,
) -> impl IntoResponse {
    println!("Uploading");
    let db = db.unwrap();

    let id_string = id.split(':').collect::<Vec<&str>>();

    // Try to extract the uploaded file (renamed from extract_image)
    let file = match extract_image(multipart).await {
        Ok(file) => file,
        Err(e) => {
            println!("File extraction failed.");
            return match e {
                small_file::Error::TooBig => (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"msg": "File is too big, use a smaller file"})),
                ),
                _ => (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"msg": "Upload Failed"})),
                ),
            };
        }
    };

    
    // Store the file in the specified directory
    dbg!(&id_string[1]);
    let path = format!("media\\video\\files\\vids\\{}", id_string[1]);

    let new_path = path.replace(".mkv", ".mp4");
    debug!("content_path-> {path}");
    let file = match storage::store(Some(path), format!("{}video", id_string[1]), file).await {
        Some(f) => f,
        None => {
            println!("File storage failed.");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"msg": "File storage error"})),
            );
        }
    };

    // Convert to mp4

    // TODO: Save output_path into DB here using `db`

    let file = (&file.0.clone(), file.1, file.2);
    let uploaded_path = file.0.clone();
    let _ = save_to_disk(file).await;
    

  
    if let Err(e) = convert_to_mp4(uploaded_path.to_str().unwrap(), &new_path).await {
        eprintln!("Video conversion failed: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"msg": "Video conversion failed"})),
        );
    }
    //let value = new_path;
    //debug!("{value}");

    let payload = ContentForUpdate {
        field: "file".to_string(),
        value: new_path.to_string(),
    };

    println!("{:?}", &new_path);

    match update(&db, id_string[1], payload).await {
        Ok(_) => (StatusCode::CREATED, Json(json!({"msg": "File uploaded"}))),
        Err(e) => {
            dbg!(e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"msg": "Upload error"})),
            )
        }
    }
}


async fn video_stream(
    //State(db): State<Db>,
    AxumPath(filepath): AxumPath<String>,
) -> impl IntoResponse{
    //let db = db.unwrap();
    //let filepath = format!("media/video/files/vids"); //Add path to filename
    println!("Serving video {filepath}");

    match File::open(filepath).await{
        Ok(file) => {
            let stream = ReaderStream::new(file);
            let body = Body::from_stream(stream);

            let mut headers = HeaderMap::new();
            headers.insert(
                header::CONTENT_TYPE,
                "video/x-matroska".parse().unwrap(), // 👈 important
            );
            (StatusCode::OK, headers, body).into_response()
        },
        Err(_) =>(StatusCode::NOT_FOUND).into_response()
    }
}
// endregion:   --- Router handlers
// region:      ---
// endregion:   ---