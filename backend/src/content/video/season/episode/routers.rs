use std::path::PathBuf;

// region:      --- Imports
use axum::{
    body::Body,
    extract::{Multipart, Path as AxumPath, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use http::Response;
use http::{HeaderMap, StatusCode};
use hyper::header;
use serde_json::json;
use tokio::fs::File;
use tracing::debug;
use std::{fs::File as StdFile, io::SeekFrom};
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use tokio_util::io::ReaderStream;

use crate::{
    content::{
        video::season::episode::{
            controllers::{index, show, store, update},
            routers::file_handlers::convert_to_mp4,
        },
        ContentForUpdate, EpisodeForCreate,
    },
    dev_initial::db::Db,
    file_upload::{
        small_file::{self, extract_image},
        storage::{self, save_to_disk},
    },
};
const CHUNK_SIZE: u64 = 1024 * 1024;
mod file_handlers;
// endregion:   --- Imports

// region:      --- Router
pub fn episode_router() -> Router<Db> {
    Router::new()
        .route("/file/*file_path", get(video_stream))
        .route("/show/:chapter", get(episode_get))
        .route("/video/upload/:id", post(video_upload))
        .route("/:season", get(episode_list))
        .route("/", post(episode_create))
}
// endregion:   --- Router

// region:      --- Router handlers
async fn episode_create(
    State(db): State<Db>,
    Json(payload): Json<EpisodeForCreate>,
) -> impl IntoResponse {
    let db = db.unwrap();
    debug!("content/episode/episode_create -> {:#?}", payload);

    match store(&db, payload).await {
        Ok(episode) => (StatusCode::CREATED, Json(json!({"episode":episode}))).into_response(),
        Err(error) => {
            dbg!(error);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}

async fn episode_get(
    State(db): State<Db>,
    AxumPath(episode): AxumPath<String>,
) -> impl IntoResponse {
    let db = db.unwrap();
    eprintln!("GET EPISODE");

    match show(&db, episode).await {
        Ok(episode) => (StatusCode::OK, Json(json!({"episode": episode}))),
        Err(_e) => (
            StatusCode::NOT_FOUND,
            Json(json!({"msg": "Episode is not found"})),
        ),
    }
}

async fn episode_list(
    State(db): State<Db>,
    AxumPath(season): AxumPath<String>,
) -> impl IntoResponse {
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
async fn episode_upate(State(db): State<Db>) -> impl IntoResponse {}
async fn episode_delete(State(db): State<Db>) -> impl IntoResponse {}

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
    let db = db.unwrap();

    let id_string = id.split(':').collect::<Vec<&str>>();

    // Extract uploaded file
    let file = match extract_image(multipart).await {
        Ok(file) => file,
        Err(e) => {
            eprintln!("File extraction failed.");
            return match e {
                small_file::Error::TooBig => (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"msg": "File is too big"})),
                ),
                _ => (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"msg": "Upload failed"})),
                ),
            };
        }
    };

    // Store file to disk
    let storage_dir = format!("media\\video\\files\\vids\\{}", id_string[1]);
    let filename = format!("{}video.mkv", id_string[1]); // force mkv extension here
    debug!("content_path -> {}", storage_dir);

    let file = match storage::store(Some(storage_dir.clone()), filename.clone(), file).await {
        Some(f) => f,
        None => {
            eprintln!("File storage failed.");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"msg": "File storage error"})),
            );
        }
    };

    let file = (&file.0.clone(), file.1, file.2);
    let uploaded_path = file.0.clone();
    let _ = save_to_disk(file).await.unwrap();

    // Build new output path
    let output_path = uploaded_path.with_extension("mp4"); // replaces mkv with mp4 safely

    // Convert MKV to MP4
    if let Err(e) = convert_to_mp4(
        uploaded_path.to_str().unwrap(),
        output_path.to_str().unwrap(),
    )
    .await
    {
        eprintln!("Video conversion failed: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"msg": "Video conversion failed"})),
        );
    }

    // Update DB with new mp4 file path
    let payload = ContentForUpdate {
        field: "file".to_string(),
        value: output_path.to_string_lossy().to_string(),
    };

    if let Err(e) = tokio::fs::remove_file(&uploaded_path).await {
        eprintln!("Failed to delete original file: {}", e);
    };
    
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
    AxumPath(file): AxumPath<String>,
    headers: HeaderMap,
) -> impl IntoResponse {

    let path = PathBuf::from(file);
    // if let Ok(file) = tokio::fs::File::open(&file_path).await {
    //     let stream = tokio_util::io::ReaderStream::new(file);
    //     Response::builder()
    //         .header("Content-Type", "video/mp4")
    //         .body(Body::from_stream(stream))
    //         .unwrap()
    // } else {
    //     (StatusCode::NOT_FOUND, "File not found").into_response()
    // }

    // Check if file exists
    let metadata = match tokio::fs::metadata(&path).await {
        Ok(meta) => meta,
        Err(_) => return StatusCode::NOT_FOUND.into_response(),
    };
    let file_size = metadata.len();

    // Parse Range header if present
    let range_header = headers.get("Range").and_then(|val| val.to_str().ok());
    let (start, end) = if let Some(range_header) = range_header {
        if let Some(range) = range_header.strip_prefix("bytes=") {
            let parts: Vec<&str> = range.split('-').collect();
            let start = parts.get(0).and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
            let end = parts
                .get(1)
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or((start + CHUNK_SIZE - 1).min(file_size - 1));
            (start, end.min(file_size - 1))
        } else {
            (0, file_size - 1)
        }
    } else {
        (0, file_size - 1)
    };

    let chunk_size = end - start + 1;

    // Open file and seek to start position
    let mut file = match tokio::fs::File::open(&path).await {
        Ok(f) => f,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    if let Err(_) = file.seek(SeekFrom::Start(start)).await {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    // Limit how much we read
    let stream = ReaderStream::new(file.take(chunk_size));

    // Build partial content response
    let mut response = Response::builder()
        .status(StatusCode::PARTIAL_CONTENT)
        .header("Content-Type", "video/mp4")
        .header("Accept-Ranges", "bytes")
        .header(
            "Content-Range",
            format!("bytes {}-{}/{}", start, end, file_size),
        )
        .body(Body::from_stream(stream))
        .unwrap();

    response
}
// endregion:   --- Router handlers
// region:      ---
// endregion:   ---
