use std::path::Path;

// section:      -- imports
use axum::body::{to_bytes, Body};
use axum::extract::{Multipart, Path as AxumPath, Request, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{middleware, routing::*, Json, Router};

use http::header;
use serde_json::json;

use tower_cookies::{Cookie, Cookies};

use crate::creator::controllers::delete_creator;
use crate::creator::CreatorForLoginSuccess;
use crate::dev_initial::db::Db;
use crate::file_upload::small_file::{self, extract_image};
use crate::file_upload::storage::{save_to_disk, store};
use crate::middleware::auth::cookies::{gen_auth_cookie, gen_refresh_cookie, verify_user};

use super::controllers::{get_details, get_studios, login, register, update_details};
use super::{CreatorForCreate, CreatorForLogin, CreatorForUpdateClient};

// endsection:   -- imports

// section:      -- router
pub fn creator_router() -> Router<Db> {
    Router::new()
        .route("/upload/:email", patch(avatar_upload))
        .route("/image/*path", get(get_image))
        .route(
            "/",
            get(details_handler)
                .patch(details_update_handler)
                .delete(delete_handler),
        )
        .layer(middleware::from_fn(verify_user))
        .route("/studio/:id", get(list_studios_handler))
        .route("/", post(register_handler))
        .route("/login", post(login_handler))
}

// endsection:   -- router

// section:      -- handlers
// #[axum_macros::debug_handler]
/// <h1> Handles registration of Creator </h1>
/// <h2> <b>Endpoint:  <strong>[POST]</strong>  /creator </b> </h2>
///
/// <h3> Request body</h3>
/// { <br>
///     "username": "test_creator1", <br>
///     "email": "test_creator1@gmail.com", <br>
///     "role": ["writer", "artist"], <br>
///     "password": "password" <br>
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
async fn register_handler(
    State(db): State<Db>,
    Json(payload): Json<CreatorForCreate>,
) -> impl IntoResponse {
    let db = db.unwrap();
    let creator = register(&db, payload).await;

    match creator {
        Ok(_c) => (StatusCode::CREATED).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

#[axum_macros::debug_handler]
/// <h1> Handles logging in of Creator </h1>
/// <h2> <b>Endpoint: <strong>[GET]</strong>  /creator </b> </h2>
///
/// <h3> Request body</h3>
/// { <br>
///     "email": "test_creator1@gmail.com", <br>
///     "password": "password" <br>
/// }<br><br>
///
/// <p>
///     Parameters cannot be empty
/// </p>
///
/// <br><hr>
/// <h4>Status Codes</h4>
/// <ul>
///     <li> <b>Ok</b>  : 202</li>
///     <li> <b>Err</b> : 401, 500, 403</li>
/// </ul>
async fn login_handler(
    State(db): State<Db>, //TODO: Update the status codes of each rout
    cookies: Cookies,
    Json(payload): Json<CreatorForLogin>,
) -> impl IntoResponse {
    let db = db.unwrap();
    let creator = login(&db, payload).await;
    // Add a not found error
    match creator {
        Ok(claims) => {
            match gen_auth_cookie(&claims, &cookies) {
                // Generating the auth token and saving it as a cookie then handling the error
                //TODO: Check if many people log in at the same time,does it affect genaration of auth cookie
                Ok(_) => {
                    if let Ok(_refresh_token) = gen_refresh_cookie(&claims, &db).await {
                        //Generate refresh token and save it to db
                    }
                }
                Err(_) => {
                    return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
                }
            }
            (StatusCode::ACCEPTED, Json(json!({"creator":claims}))).into_response()
        }
        Err(e) => match e {
            crate::creator::CreatorError::LoginError => {
                (StatusCode::INTERNAL_SERVER_ERROR).into_response()
            }
            crate::creator::CreatorError::WrongCredentialsError => {
                (StatusCode::UNAUTHORIZED).into_response()
            }
            crate::creator::CreatorError::LoginAttemptsError => (
                StatusCode::FORBIDDEN,
                Json(json!({"msg":"Too many login attempts"})),
            )
                .into_response(),
            _ => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        },
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
pub async fn details_handler(State(db): State<Db>, req: Request) -> impl IntoResponse {
    //TODO: Change to path
    if let Some(id) = req.extensions().get::<String>() {
        let db = db.unwrap();
        let creator = get_details(&db, id.to_owned()).await;
        match creator {
            Ok(c) => (StatusCode::OK, Json(json!({"creator": c}))),
            Err(_) => (
                StatusCode::NOT_FOUND,
                Json(json!({"msg": "Creator not found"})),
            ),
        }
    } else {
        println!("Error");
        (
            StatusCode::NOT_FOUND,
            Json(json!({"msg": "Creator not found"})),
        )
    }
}

// #[axum_macros::debug_handler]
// pub async fn details_update_handler(State(db): State<Db>, Json(payload):Json<Vec<CreatorForUpdate>>) -> impl IntoResponse{
#[axum_macros::debug_handler]
/// <h1> Handles updating details of Creators </h1>
/// <h2> <b>Endpoint:  <strong>[PATHC]</strong>  /creator </b> </h2>
///
/// <h3> Request body</h3>
/// { <br>
///     "field": "email", <br>
///     "value": "test_creator1@gmail.com", <br>
/// }<br><br>
///
/// <p>
///     Empty parameters <br>
///     Need auth token <br>
/// </p>
///
/// <br><hr>
/// <h4>Status Codes</h4>
/// <ul>
///     <li> <b>Ok</b>  : 302</li>
///     <li> <b>Err</b> : 500</li>
/// </ul>
pub async fn details_update_handler(State(db): State<Db>, req: Request) -> impl IntoResponse {
    return if let Some(req_id) = req.extensions().get::<String>() {
        // Get users id
        let id = req_id.clone(); // It's cloned since request is consumed in the next section

        // the 2 lines below extract request body and serialize it into the correct format
        let body_bytes: axum::body::Bytes = to_bytes(req.into_body(), 2480).await.unwrap();
        let payload: CreatorForUpdateClient = serde_json::from_slice(&body_bytes).unwrap();

        dbg!(&payload);
        let db = db.unwrap(); //select db
        let creator = update_details(&db, id.to_owned(), payload).await;

        // Items to update username, password, socials, description,
        match creator {
            Ok(_) => {
                let creator = get_details(&db, id.to_owned()).await.unwrap();
                (StatusCode::OK, Json(json!({"creator": creator})))
            }
            Err(_) => {
                let creator = get_details(&db, id.to_owned()).await.unwrap();
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"msg": "Update error", "c":creator})),
                )
            }
        }
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"msg": "Creator not updated"})),
        )
    };
}

/// <h1> Handles deleting of Creators </h1>
/// <h2> <b>Endpoint:  <strong>[DELETE]</strong>  /creator </b> </h2>
///
/// <h3> No request body</h3>
///
/// <p>
///     Empty parameters <br>
///     Need auth token <br>
/// </p>
///
///<br> <hr>
/// <h4>Status Codes</h4>
/// <ul>
///     <li> <b>Ok</b>  : 200</li>
///     <li> <b>Err</b> : 500</li>
/// </ul>
pub async fn delete_handler(
    State(db): State<Db>,
    cookies: Cookies,
    req: Request,
) -> impl IntoResponse {
    if let Some(id) = req.extensions().get::<String>() {
        let db = db.unwrap();
        let creator = delete_creator(&db, id.to_owned()).await;
        match creator {
            Ok(c) => {
                cookies.remove(Cookie::from("auth_token"));
                (StatusCode::OK, Json(json!({"creator": c})))
            }
            Err(_) => (
                StatusCode::NOT_FOUND,
                Json(json!({"msg": "Creator not found"})),
            ),
        }
    } else {
        println!("Error");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"msg": "Creator not found"})),
        )
    }
}

#[axum::debug_handler]
async fn avatar_upload(
    State(db): State<Db>,
    email: AxumPath<String>,
    multipart: Multipart,
) -> impl IntoResponse {
    let query = format!("SELECT * FROM creator WHERE email = '{}' ", &email[..]);
    // };s
    println!("{:?}", query);
    let db = db.unwrap();
    // let id: Option<Thing> = db.select(("table",email)).await.unwrap();

    let mut t = db
        .clone()
        .query(query)
        .bind(("table", "creator"))
        .await
        .unwrap();
    let id: Option<CreatorForLoginSuccess> = t.take(0).unwrap();
    let mut creator_id = String::new();
    let id = match id {
        Some(c) => {
            creator_id = c.id.id.clone().to_string();
            format!("{}:{}", c.id.tb, c.id.id)
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

    let file = match store(
        Some(format!("media\\user\\creator\\{}", creator_id)),
        format!("{}profile_pic", creator_id),
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

    let payload = CreatorForUpdateClient {
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
pub async fn list_studios_handler(State(db): State<Db>, AxumPath(id): AxumPath<String>, req: Request) -> impl IntoResponse {
    //TODO: Check if the creator exists
    let mut id = id;
    if id == "owner"{
        if let Some(i) = req.extensions().get::<String>() {
            id = i.to_string();
        }
    }
    let db = db.unwrap();
    let studios = get_studios(&db, id).await;
    dbg!(&studios);
    if let Ok(s) = studios {
        (StatusCode::OK, Json(json!({"studios": s}))).into_response()
    } else {
        (StatusCode::NOT_FOUND).into_response()
    }
}
// endsection:   -- handlers
