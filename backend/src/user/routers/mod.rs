// section:      -- imports
use axum::body::to_bytes;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{middleware, routing::*, Json, Router};
use serde_json::json;
use tower_cookies::{Cookie, Cookies};

use crate::dev_initial::db::Db;
use crate::middleware::auth::cookies::{gen_auth_cookie, gen_refresh_cookie, verify_user};
use crate::user::controllers::delete_user;

use super::controllers::{get_details, login, register, update_details};
use super::{UserForCreate, UserForLogin, UserForUpdateClient};

// endsection:   -- imports

// section:      -- router
pub fn user_router() -> Router<Db> {
    Router::new()
        .route(
            "/",
            get(details_handler)
                .patch(details_update_handler)
                .delete(delete_handler),
        )
        .layer(middleware::from_fn(verify_user))
        .route("/", post(register_handler))
        .route("/login", get(login_handler))
}

// endsection:   -- router

// section:      -- handlers
// #[axum_macros::debug_handler]
/// <h1> Handles registration of User </h1>
/// <h2> <b>Endpoint:  <strong>[POST]</strong>  /User </b> </h2>
///
/// <h3> Request body</h3>
/// { <br>
///     "username": "test_User1", <br>
///     "email": "test_User1@gmail.com", <br>
///     "role": ["writer", "artist"], <br>
///     "password": "password" <br>
/// }<br><br>
///
/// <h5>
///     Parameters cannot be empty
/// </h5>
async fn register_handler(
    State(db): State<Db>,
    Json(payload): Json<UserForCreate>,
) -> impl IntoResponse {
    let db = db.unwrap();
    let user = register(&db, payload).await;

    match user {
        Ok(_c) => (StatusCode::CREATED).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

#[axum_macros::debug_handler]
/// <h1> Handles logging in of User </h1>
/// <h2> <b>Endpoint: <strong>[GET]</strong>  /User </b> </h2>
///
/// <h3> Request body</h3>
/// { <br>
///     "email": "test_User1@gmail.com", <br>
///     "password": "password" <br>
/// }<br><br>
///
/// <h5>
///     Parameters cannot be empty
/// </h5>
async fn login_handler(
    State(db): State<Db>,
    cookies: Cookies,
    Json(payload): Json<UserForLogin>,
) -> impl IntoResponse {
    dbg!(&payload);
    let db = db.unwrap();
    let user = login(&db, payload).await;

    match user {
        Ok(claims) => {
            match gen_auth_cookie(&claims, &cookies) {
                // Generating the auth token and saving it as a cookie then handling the error
                Ok(_) => {
                    if let Ok(refresh_token) = gen_refresh_cookie(&claims, &db).await {
                        //Generate refresh token and save it to db
                        println!("{:?}", refresh_token);
                    }
                }
                Err(_) => {
                    return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
                }
            }
            (StatusCode::ACCEPTED, Json(json!({"ser":claims}))).into_response()
        }
        Err(e) => match e {
            crate::user::UserError::LoginError => {
                (StatusCode::INTERNAL_SERVER_ERROR).into_response()
            }
            crate::user::UserError::WrongCredentialsError => {
                (StatusCode::UNAUTHORIZED).into_response()
            }
            crate::user::UserError::LoginAttemptsError => (
                StatusCode::FORBIDDEN,
                Json(json!({"msg":"Too many login attempts"})),
            )
                .into_response(),
            _ => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        },
    }
}

/// <h1> Handles getting details of User </h1>
/// <h2> <b>Endpoint: /User </b> </h2>
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
        let user = get_details(&db, id.to_owned()).await;
        match user {
            Ok(c) => (StatusCode::FOUND, Json(json!({"User": c}))),
            Err(_) => (
                StatusCode::NOT_FOUND,
                Json(json!({"msg": "User not found"})),
            ),
        }
    } else {
        println!("Error");
        (
            StatusCode::NOT_FOUND,
            Json(json!({"msg": "User not found"})),
        )
    }
}

// #[axum_macros::debug_handler]
// pub async fn details_update_handler(State(db): State<Db>, Json(payload):Json<Vec<UserForUpdate>>) -> impl IntoResponse{
#[axum_macros::debug_handler]
/// <h1> Handles updating details of Users </h1>
/// <h2> <b>Endpoint:  <strong>[PATHC]</strong>  /User </b> </h2>
///
/// <h3> Request body</h3>
/// { <br>
///     "field": "email", <br>
///     "value": "test_User1@gmail.com", <br>
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
        let body_bytes = to_bytes(req.into_body(), 2480).await.unwrap();
        let payload: UserForUpdateClient = serde_json::from_slice(&body_bytes).unwrap();

        let db = db.unwrap(); //select db
        let user = update_details(&db, id.to_owned(), payload).await;

        // Items to update username, password, socials, description,
        match user {
            Ok(_) => {
                let user = get_details(&db, id.to_owned()).await.unwrap();
                (StatusCode::FOUND, Json(json!({"user": user})))
            }
            Err(_) => {
                let user = get_details(&db, id.to_owned()).await.unwrap();
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"msg": "Update error", "user": user})),
                )
            }
        }
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"msg": "User not updated"})),
        )
    };
}

/// <h1> Handles deleting of Users </h1>
/// <h2> <b>Endpoint:  <strong>[DELETE]</strong>  /User </b> </h2>
///
/// <h3> No request body</h3>
///
/// <h5>
///     Empty parameters <br>
///     Need auth token <br>
/// </h5>
///
pub async fn delete_handler(
    State(db): State<Db>,
    cookies: Cookies,
    req: Request,
) -> impl IntoResponse {
    if let Some(id) = req.extensions().get::<String>() {
        let db = db.unwrap();
        let user = delete_user(&db, id.to_owned()).await;
        match user {
            Ok(c) => {
                cookies.remove(Cookie::from("auth_token"));
                (StatusCode::OK, Json(json!({"User": c})))
            }
            Err(_) => (
                StatusCode::NOT_FOUND,
                Json(json!({"msg": "User not found"})),
            ),
        }
    } else {
        println!("Error");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"msg": "User not found"})),
        )
    }
}

// endsection:   -- handlers

// section:      -- imports
// endsection:   -- imports

// section:      -- imports
// endsection:   -- imports
