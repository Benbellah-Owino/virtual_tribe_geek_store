// section:      -- imports
use axum::body::to_bytes;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{middleware, routing::*, Json, Router};
use serde_json::json;
use tower_cookies::{Cookie, Cookies};

use crate::creator::controllers::delete_creator;
use crate::dev_initial::db::Db;
use crate::middleware::auth::cookies::{gen_auth_cookie, gen_refresh_cookie, verify_user};

use super::controllers::{get_details, login, register, update_details};
use super::{CreatorForCreate, CreatorForLogin, CreatorForUpdateClient};

// endsection:   -- imports

// section:      -- router
pub fn creator_router() -> Router<Db> {
    return Router::new()
        .route("/", get(details_handler).patch(details_update_handler).delete(delete_handler))
        .layer(middleware::from_fn(verify_user))
        .route("/", post(register_handler))
        .route("/login", get(login_handler));
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
/// <h5>
///     Parameters cannot be empty
/// </h5>
async fn register_handler(
    State(db): State<Db>,
    Json(payload): Json<CreatorForCreate>,
) -> impl IntoResponse {
    let db = db.unwrap();
    let creator = register(&db, payload).await;

    match creator {
        Ok(_c) => {
            return (StatusCode::CREATED).into_response();
        }
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
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
/// <h5>
///     Parameters cannot be empty
/// </h5>
async fn login_handler(
    State(db): State<Db>,
    cookies: Cookies,
    Json(payload): Json<CreatorForLogin>,
) -> impl IntoResponse {
    let db = db.unwrap();
    let creator = login(&db, payload).await;

    match creator {
        Ok(claims) => {
            match gen_auth_cookie(&claims, &cookies) {
                // Generating the auth token and saving it as a cookie then handling the error
                Ok(_) => {
                    if let Ok(_refresh_token) = gen_refresh_cookie(&claims, &db).await {
                        //Generate refresh token and save it to db
                    }
                }
                Err(_) => {
                    return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
                }
            }
            return (StatusCode::ACCEPTED, Json(json!({"creator":claims}))).into_response();
        }
        Err(e) => match e {
            crate::creator::CreatorError::LoginError => {
                return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
            }
            crate::creator::CreatorError::WrongCredentialsError => {
                return (StatusCode::UNAUTHORIZED).into_response();
            }
            crate::creator::CreatorError::LoginAttemptsError => {
                return (
                    StatusCode::FORBIDDEN,
                    Json(json!({"msg":"Too many login attempts"})),
                )
                    .into_response();
            }
            _ => {
                return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
            }
        },
    }
}

/// <h1> Handles getting details of Creator </h1>
/// <h2> <b>Endpoint: /creator </b> </h2>
///
/// <h3> No request body</h3>
///
/// <h5>
///     Empty parameters <br>
///     Need auth token <br>
/// </br>
pub async fn details_handler(State(db): State<Db>, req: Request) -> impl IntoResponse {
    if let Some(id) = req.extensions().get::<String>() {
        let db = db.unwrap();
        let creator = get_details(&db, id.to_owned()).await;
        match creator {
            Ok(c) => {
                return (StatusCode::FOUND, Json(json!({"creator": c})));
            }
            Err(_) => {
                return (
                    StatusCode::NOT_FOUND,
                    Json(json!({"msg": "Creator not found"})),
                );
            }
        }
    } else {
        println!("Error");
        return (
            StatusCode::NOT_FOUND,
            Json(json!({"msg": "Creator not found"})),
        );
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
/// <h5>
///     Empty parameters <br>
///     Need auth token <br>
/// </h5>
pub async fn details_update_handler(State(db): State<Db>, req: Request) -> impl IntoResponse {
    return if let Some(req_id) = req.extensions().get::<String>() {
        // Get users id
        let id = req_id.clone(); // It's cloned since request is consumed in the next section

        // the 2 lines below extract request body and serialize it into the correct format
        let body_bytes = to_bytes(req.into_body(), 10480).await.unwrap();
        let payload: CreatorForUpdateClient = serde_json::from_slice(&body_bytes).unwrap();

        let db = db.unwrap(); //select db
        let creator = update_details(&db, id.to_owned(), payload).await;

        // Items to update username, password, socials, description,
        match creator {
            Ok(_) => {
                let creator = get_details(&db, id.to_owned()).await.unwrap();
                (StatusCode::FOUND, Json(json!({"creator": creator})))
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
/// <h5>
///     Empty parameters <br>
///     Need auth token <br>
/// </h5>
/// 
pub async fn delete_handler(State(db): State<Db>, cookies:Cookies, req: Request) -> impl IntoResponse {
    if let Some(id) = req.extensions().get::<String>() {
        let db = db.unwrap();
        let creator = delete_creator(&db, id.to_owned()).await;
        match creator {
            Ok(c) => {
                cookies.remove(Cookie::from("auth_token"));
                return (StatusCode::OK, Json(json!({"creator": c})));
            }
            Err(_) => {
                return (
                    StatusCode::NOT_FOUND,
                    Json(json!({"msg": "Creator not found"})),
                );
            }
        }
    } else {
        println!("Error");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"msg": "Creator not found"})),
        );
    }
}


// endsection:   -- handlers

// section:      -- imports
// endsection:   -- imports

// section:      -- imports
// endsection:   -- imports
