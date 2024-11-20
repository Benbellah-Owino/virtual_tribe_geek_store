use std::path::{self, Path};

use axum::{
    body::Body, extract::{Multipart, Path as AxumPath, State}, response::{IntoResponse, Response}, routing::get, Json, Router
};
use http::{header, StatusCode};
use serde_json::json;
use surrealdb::sql::Thing;
use tracing::{debug, info};

use crate::{content::{controllers::update_details, ContentForCreateServer, ContentForUpdate}, dev_initial::db::Db, file_upload::{small_file::{self, extract_image}, storage::{self, save_to_disk}}};

use super::{
    controllers::{get_all, list_by_studio, store},
    genre::routers::genre_router,
};
use crate::content::ContentForCreateClient;

// region:      --- Router
pub fn content_router() -> Router<Db> {
    Router::new()
        .nest("/genre", genre_router())
        .route("/:studio", get(list_studio_handler))
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


#[axum_macros::debug_handler]
pub async fn list_studio_handler(State(db): State<Db>, AxumPath(studio): AxumPath<String>) -> impl IntoResponse {
    let db = db.unwrap();
    
    // let studio: Vec<&str> = id.split(':').collect();
    let content_list = list_by_studio(&db, studio).await;

    match content_list {
        Ok(c) => (StatusCode::OK, Json(json!({"content_list": c}))).into_response(),
        Err(e) => {
            dbg!(e);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}


#[axum::debug_handler]
async fn avatar_upload(
    State(db): State<Db>,
    AxumPath(id): AxumPath<String>,
    multipart: Multipart,
) -> impl IntoResponse {
    let db = db.unwrap();
    let id: Option<Thing> = db.select(("table",id)).await.unwrap();

    let mut content_id = String::new();
    let id = match id {
        Some(c) => {
            //content_id = c.id.id.clone().to_string();
            content_id = c.id.to_string();
            format!("{}:{}", c.tb, c.id)
        }
        None => "none".to_string(),
    };
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

    // Store the file in the specified directory

    let file = match storage::store(
        Some(format!("media\\content\\{}", content_id)),
        format!("{}cover", content_id),
        file,
    )
    .await
    {
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

    let payload = ContentForUpdate {
        field: "avatar".to_string(),
        value: value.to_string(),
    };
    
    println!("{:?}", &path);

    match update_details(&db, id, payload).await {
        Ok(_) => (StatusCode::OK, Json(json!({"msg": "File uploaded"}))),
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
pub async fn get_image(
    // State(db): State<Db>,
    AxumPath(path): AxumPath<String>,
    // req: Request,
) -> impl IntoResponse {
    //TODO: Change to path

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
