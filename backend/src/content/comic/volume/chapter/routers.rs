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
        .route("/cover/upload/:id", post(cover_upload))
        .route("/cover/*path", get(get_cover))
        .route("/file/:index/*file_path", get(get_file))
        .route("/file/info/*file_path", get(get_comic_info)) //TODO: change this path in relevant frontend fetch calls
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

/// <h1> Handles creation of Chapter </h1>
/// <h2> <b>Endpoint:  <strong>[POST]</strong>  /content/comic/volume/chapter </b> </h2>
///
/// <h3> Request body</h3>
/// { <br>
///     "pages": 1, <br>
///     "title": "Test Chapter", <br>
///     "synopsis": "this is an example of a chapter to be created", <br>
///     "volume": "volume:***", <br>
///     "cover" : "path to file" <br>
/// }<br><br>
///
/// <p>
///     Parameters can be empty
/// </p><br><hr>
///
/// <h4>Status Codes</h4>
/// <ul>
///     <li> <b>Ok: Created</b>  : 201</li>
///     <li> <b>Err: Internal Server Error</b> : 500</li>
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

/// <h1> Handles Getting list of Chapter </h1>
/// <h2> <b>Endpoint:  <strong>[GET]</strong>  /content/comic/volume/chapter/:volume </b> </h2>
///
/// <h3> Request body</h3>
///     NONE<br>
///
/// <h3> Response body</h3>
/// [{<br>
///     "id": "chapter:***", <br>
///     "relative_chapter": 5, <br>
///     "absolute_chapter": 12, <br>
///     "pages": 24, <br>
///     "sypnosis": "The heroes regroup after a devastating ambush.", <br>
///     "file": "path/to/file.pdf", <br>
///     "cover": "path/to/cover.jpg", <br>
///     "volume": "volume:***" <br>
/// }]]<br><br>
///
///     
////// <p>
///     Path parameter :volume represents volume whose chapters we want to fetch
/// </p><br><hr>
///
/// <h4>Status Codes</h4>
/// <ul>
///     <li> <b>Ok</b>  : 200</li>
///     <li> <b>Err: Not Found</b> : 404</li>
///     <li> <b>Err: Internal Server Error</b> : 500</li>
/// </ul>
#[axum_macros::debug_handler]
async fn list(State(db): State<Db>, AxumPath(volume): AxumPath<String>) -> impl IntoResponse {
    let db = db.unwrap();
    eprintln!("LIST CHAPTERS");
    //TODO: Support listing of chapters by different criteria like creation date, ratings or even by user profile
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

/// <h1> Handles Getting one instance of Chapter </h1>
/// <h2> <b>Endpoint:  <strong>[GET]</strong>  /content/comic/volume/chapter/show/:chapter </b> </h2>
///
/// <h3> Request body</h3>
///     NONE <br>
///
/// <h3> Response body</h3>
/// {<br>
///     "id": "chapter:***", <br>
///     "relative_chapter": 5, <br>
///     "absolute_chapter": 12, <br>
///     "pages": 24, <br>
///     "sypnosis": "The heroes regroup after a devastating ambush.", <br>
///     "file": "path/to/file.pdf", <br>
///     "cover": "path/to/cover.jpg", <br>
///     "volume": "volume:***" <br>
/// }<br><br>
///
/// <p>
///     Path parameter id represents the Comic ID needed
///     Needs auth token
/// </p><br><hr>
///
/// <h4>Status Codes</h4>
/// <ul>
///     <li> <b>Ok</b>  : 201</li>
///     <li> <b>Err</b> : 500</li>
/// </ul>
#[axum_macros::debug_handler]
async fn get_chapter(
    State(db): State<Db>,
    // Query(chapter_query): Query<GetChapterQuery>,
    AxumPath(chapter): AxumPath<String>,
) -> impl IntoResponse {
    let db = db.unwrap();

    match show(&db, chapter).await {
        Ok(chapter) => (StatusCode::OK, Json(json!({"chapter": chapter}))),
        Err(_e) => (
            StatusCode::NOT_FOUND,
            Json(json!({"msg": "Chapter is not found"})),
        ),
    }
}

// Below Routes deal with the actual files and not db data

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
    let path = format!("media\\comic\\files\\{}", id_string[1]);
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

/// <h1> Handles the fetching of the actual comic book file </h1>
/// <h2> <b>Endpoint:  <strong>[GET]</strong>  /content/comic/volume/chapter/comic/file/:index/*file_path </b> </h2>
///
/// <h3> Request body</h3>
///     NONE <br>
///
/// <h3> Request body</h3>
///     FILE DATA <br>
///
/// <p>
///     Path parameter :index represents the page to be fetched
///     Path parameter *file represents the path to comic book file
///     Needs auth token
/// </p><br><hr>
///
/// <h4>Status Codes</h4>
/// <ul>
///     <li> <b>Ok</b>  : 200</li>
///     <li> <b>Err: Not Found</b> : 404</li>
///     <li> <b>Err: Unsupported Media Type </b> : 415</li>
///     <li> <b>Err: Internal Server Error</b> : 500</li>
pub async fn get_file(
    AxumPath((index, file_path)): AxumPath<(usize, String)>,
) -> impl IntoResponse {
    eprintln!("{}", file_path);
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

/// <h1> Handles Getting one instance of Chapter </h1>
/// <h2> <b>Endpoint:  <strong>[GET]</strong>  /content/comic/volume/chapter/file/info/*file_path </b> </h2>
///
/// <h3> Request body</h3>
///     NONE <br>
/// <h3> Request body</h3>
/// {<br>
///     "count": 10, <br>
///     "content_type": "pdf"br <br>
/// }<br><br>
///
/// <p>
///     Path parameter *file_path represents path to file
///     Needs auth token
/// </p><br><hr>
///
/// <h4>Status Codes</h4>
/// <ul>
///     <li> <b>Ok</b>  : 200</li>
///     <li> <b>Err: Bad Request</b> : 400</li>
///     <li> <b>Err: Not Found</b> : 404</li>
///     <li> <b>Err: Internal Server Error</b> : 500</li>
/// </ul>
#[axum_macros::debug_handler]
async fn get_comic_info(AxumPath(path): AxumPath<String>) -> impl IntoResponse {
    // Get pages count
    let content_type = Path::new(&path).extension().and_then(|ext| ext.to_str());

    let ct: String = if content_type.is_some() {
        content_type.unwrap().to_string()
    } else {
        String::from("error")
    };

    let mut count = 0;

    match content_type {
        Some("octet-stream") | Some("zip") => {
            let file = match File::open(&path) {
                Ok(f) => f,
                Err(_e) => {
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

/// <h1> Handles the uploading of the chapter cover photo</h1>
/// <h2> <b>Endpoint:  <strong>[POST]</strong>  /content/comic/volume/chapter/cover/upload/:id </b> </h2>
///
/// <h3> Request body</h3>
///     "FILE DATA" <br>
///
/// <p>
///      Path parameter id represents the id of Chapter whose cover photo is being uploaded
/// </p><br><hr>
///
/// <h4>Status Codes</h4>
/// <ul>
///     <li> <b>Ok: Created</b>  : 201</li>
///     <li> <b>Err: Bad Request</b> : 400</li>
///     <li> <b>Err: Internal Server Error</b> : 500</li>
/// </ul>
#[axum::debug_handler]
async fn cover_upload(
    State(db): State<Db>,
    AxumPath(id): AxumPath<String>,
    multipart: Multipart,
) -> impl IntoResponse {
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
                        Json(json!({"msg": "Image is too big, use a smaller image"})),
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
    let path = format!("media\\comic\\covers\\{}", id_string[1]);
    debug!("content_path-> {path}");
    let file = match storage::store(Some(path), format!("{}cover", id_string[1]), file).await {
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
        field: "cover".to_string(),
        value: value.to_string(),
    };

    println!("{:?}", &path);

    match update(&db, id_string[1], payload).await {
        Ok(_) => (StatusCode::CREATED, Json(json!({"msg": "Cover uploaded"}))),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"msg": "Upload error"})),
        ),
    }
}

/// <h1> Handles getting details of content </h1>
/// <h2> <b>Endpoint: /content </b> </h2>
///
/// <h3> No request body</h3>
///
/// <h3> Request body</h3>
///     NONE <br>
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
///
///
pub async fn get_cover(AxumPath(path): AxumPath<String>) -> impl IntoResponse {
    println!("{:?}", path);
    let path = path.to_string();
    match tokio::fs::read(path.clone()).await {
        Ok(d) => {
            let content_type = match Path::new(&path).extension().and_then(|ext| ext.to_str()) {
                Some("png") => "image/png",
                Some("jpg") | Some("jpeg") => "image/jpeg",
                Some("gif") => "image/gif",
                Some("bmp") => "image/bmp",
                Some("webp") => "image/webp",
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
