// region:      --- Imports
use axum::{extract::State, response::IntoResponse, routing::get, Json, Router};
use http::StatusCode;
use serde_json::json;

use crate::dev_initial::db::Db;

use super::{
    controllers::{list, store},
    GenreForCreate,
};
// endregion:   --- Imports

// region:      --- RouterFunction
pub fn genre_router() -> Router<Db> {
    Router::new().route("/", get(list_handler).post(create_handler))
}
// endregion:   --- RouterFunction

// region:      --- Handlers

//TODO: Make this admin only function
/// <h1> Handles creation of Genre </h1>
/// <h2> <b>Endpoint:  <strong>[POST]</strong>  /content/genre </b> </h2>
///
/// <h3> Request body</h3>
/// { <br>
///     "name": "fantasy", <br>
///     "description": "A thrilling adventure of unlikely heroes.", <br>
/// }<br><br>
///
/// <p>
///     Parameters are empty
/// </p><br><hr>
///
/// <h4>Status Codes</h4>
/// <ul>
///     <li> <b>Ok: Created</b>  : 201</li>
///     <li> <b>Err: Internal Server Error</b> : 500</li>
/// </ul>
pub async fn create_handler(
    State(db): State<Db>,
    Json(payload): Json<GenreForCreate>,
) -> impl IntoResponse {
    let db = db.unwrap();
    let new_genre = store(payload, &db).await;

    match new_genre {
        Ok(g) => (StatusCode::CREATED, Json(json!({"genre": g}))).into_response(),
        Err(e) => {
            dbg!(e);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}

/// <h1> Handles Getting list of Genre </h1>
/// <h2> <b>Endpoint:  <strong>[GET]</strong>  /content/genre </b> </h2>
///
/// <h3> Request body</h3>
///         NONE<br>
/// 
/// <h3> Response body</h3>
/// [{ <br>
///     "name": "fantasy", <br>
///     "description": "A thrilling adventure of unlikely heroes.", <br>
/// }, ...]<br><br>
/// 
/// 
/// <p>
///     Parameters can be empty
/// </p><br><hr>
///
/// <h4>Status Codes</h4>
/// <ul>
///     <li> <b>Ok</b>  : 200</li>
///     <li> <b>Err: Internal Server Error</b> : 500</li>
/// </ul>
pub async fn list_handler(State(db): State<Db>) -> impl IntoResponse {
    let db = db.unwrap();
    let genre_list = list(&db).await;

    match genre_list {
        Ok(g) => (StatusCode::OK, Json(json!({"genres": g}))).into_response(),
        Err(e) => {
            dbg!(e);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
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
