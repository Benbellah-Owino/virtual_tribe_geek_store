use crate::content::comic::volume::chapter::routers::file_handlers::{open_octet_stream, open_pdf};
use crate::content::ContentForUpdate;
use crate::{
    content::comic::volume::chapter::{
        controllers::{index, show, store, update},
        ChapterForCreate,
    },
    dev_initial::db::Db,
    file_upload::{
        small_file::{self, extract_image},
        storage::{self, save_to_disk},
    },
};
use axum::body::Body;
use axum::extract::Multipart;
use axum::response::Response;
use axum::{
    extract::{Path as AxumPath, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use http::{header, StatusCode};
use serde::Deserialize;
use serde_json::json;
use std::fs::File;
use std::path::Path;
use tracing::debug;
use zip::ZipArchive;

mod file_handlers;

pub fn chapter_router() -> Router<Db> {
    return Router::new()
        .route("/file/upload/:id", post(file_upload))
        .route("/:volume", get(list))
        .route("/show/:chapter", get(get_chapter))
        //.route("/image/*path", get(get_file))
        .route("/file/:index/*file_path", get(get_file))
        .route("/file/count/*file_path", get(get_comic_info))
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
        }
    }
}

/// <h1> Handles getting details of Creator </h1>
/// <h2> <b>Endpoint: /creator </b> </h2>
///
/// <h3> No request body</h3>
///
/// <p>
///     Empty parameters <br>
///     Need auth token <br>
/// </p>
/// <br><hr>
/// <h4>Status Codes</h4>
/// <ul>
///     <li> <b>Ok</b>  : 302</li>
///     <li> <b>Err</b> : 404</li>
/// </ul>
pub async fn send_file(
    // State(db): State<Db>,
    AxumPath(path): AxumPath<String>,
    // req: Request,
) -> impl IntoResponse {
    //TODO: Change to path

    println!("{:?}", path);
    let path = path.to_string();
    match tokio::fs::read(path.clone()).await {
        Ok(d) => {
            let content_type = Path::new(&path).extension().and_then(|ext| ext.to_str());
            eprintln!("CONTENT TYPE: {:#?}", content_type);
            let content_type = match content_type {
                Some("png") => "image/png",
                Some("jpg") | Some("jpeg") => "image/jpeg",
                Some("cbz") | Some("cbr") | Some("zip") | Some("octet-stream") => {
                    "application/octet-stream"
                }
                Some("pdf") => "application/pdf",
                _ => {
                    return (
                        StatusCode::UNSUPPORTED_MEDIA_TYPE,
                        "Unsupported image format",
                    )
                        .into_response()
                }
            };
            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, content_type)
                .body(Body::from(d))
                .unwrap()
        }
        Err(_) => todo!(),
    }
}

pub async fn get_file(
    // State(db): State<Db>,
    AxumPath((index, file_path)): AxumPath<(usize, String)>,
    // req: Request,
) -> impl IntoResponse {
    //TODO: Handle PDFS files
    eprintln!("{}", file_path);
    //let archive_path = format!("media/comics/{}/{}", chapter,file_path); // example: "comics/mycomic.cbz"
    //let archive_path = format!("{}", file_path); // example: "comics/mycomic.cbz"
    let content_type = Path::new(&file_path)
        .extension()
        .and_then(|ext| ext.to_str());
    eprintln!("CONTENT TYPE: {:#?}", content_type);
    match content_type {
        Some("png") => {
            unimplemented!()
        }
        Some("jpg") | Some("jpeg") => {
            unimplemented!()
        }
        Some("cbz") | Some("cbr") => {
            unimplemented!()
        }
        Some("zip") | Some("octet-stream") => {
            let (comic_file, mime) = match open_octet_stream(file_path, index) {
                Ok(c) => c,
                Err(e) => {
                    dbg!(e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Could not retrieve file")
                        .into_response();
                }
            };

            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, mime.as_ref())
                .body(Body::from(comic_file))
                .unwrap()
        }
        Some("pdf") => match open_pdf(file_path).await {
            Ok(comic_file) => Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/pdf")
                .body(Body::from(comic_file))
                .unwrap(),
            Err(_) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, "Could not retrieve file")
                    .into_response()
            }
        },
        _ => {
            return (
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                "Unsupported image format",
            )
                .into_response()
        }
    }
}

#[axum_macros::debug_handler]
async fn get_comic_info(AxumPath(path): AxumPath<String>) -> impl IntoResponse {
    // Get pages count
    let content_type = Path::new(&path).extension().and_then(|ext| ext.to_str());
    let mut ct = String::new();
    if let Some(content_type) = content_type {
        ct = content_type.to_owned();
    } else {
        ct = String::from("error")
    };

    let mut count = 0;

    match content_type {
        Some("octet-stream") | Some("zip") => {
            let file = match File::open(&path) {
                Ok(f) => f,
                Err(e) => {
                    return (axum::http::StatusCode::NOT_FOUND, "File not found").into_response()
                }
            };

            let mut archive = match ZipArchive::new(file) {
                Ok(a) => a,
                Err(_) => {
                    return (axum::http::StatusCode::BAD_REQUEST, "Invalid archive").into_response()
                }
            };

            for i in 0..archive.len() {
                if let Ok(file) = archive.by_index(i) {
                    let name = file.name();
                    if let Some(ext) = Path::new(name).extension().and_then(|e| e.to_str()) {
                        if matches!(
                            ext.to_ascii_lowercase().as_str(),
                            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp"
                        ) {
                            count += 1;
                        }
                    }
                }
            }
        }
        _ => {
            // Get page count from Database for other file types
            count = 0;
        }
    }
    // Get comic file type

    return (
        StatusCode::OK,
        Json(json!({
            "count": count,
            "content_type" : ct
        })),
    )
        .into_response();
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
async fn get_chapter(
    State(db): State<Db>,
    // Query(chapter_query): Query<GetChapterQuery>,
    AxumPath(chapter): AxumPath<String>,
) -> impl IntoResponse {
    let db = db.unwrap();

    match show(&db, chapter).await {
        Ok(chapter) => (StatusCode::OK, Json(json!({"chapter": chapter}))),
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(json!({"msg": "Chapter is not found"})),
        ),
    }
}
