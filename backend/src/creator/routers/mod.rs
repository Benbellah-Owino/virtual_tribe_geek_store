// section:      -- imports
use axum::{
    Json,
    Router,
    routing::*,
};
use axum::response::IntoResponse;
use axum::extract::State;
use axum::http::StatusCode;
use serde_json::json;


use crate::dev_initial::db::Db;

use super::controllers::{
    register,
    login
};
use super::{CreatorForCreate, CreatorForLogin};
// endsection:   -- imports



// section:      -- router
pub fn creator_router() -> Router<Db>{
    return Router::new()
        .route("/", post(register_handler))
        .route("/", get(login_handler))
}
// endsection:   -- router



// section:      -- handlers
#[axum_macros::debug_handler]
async fn register_handler(State(db): State<Db>, Json(payload): Json<CreatorForCreate>) -> impl IntoResponse{
    dbg!(&payload);
    let db = db.unwrap();
    let creator = register(&db, payload).await;

    match creator{
        Ok(_c) => {
            return (StatusCode::CREATED).into_response() 
        },
        Err(_e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}



#[axum_macros::debug_handler]
async fn login_handler(State(db): State<Db>, Json(payload): Json<CreatorForLogin>) -> impl IntoResponse{
    dbg!(&payload);
    let db = db.unwrap();
    let creator = login(&db, payload).await;

    match creator{
        Ok(c) => {
            return (StatusCode::ACCEPTED, Json(json!({"creator":c}))).into_response() 
        },
        Err(_e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}
// endsection:   -- handlers



// section:      -- imports
// endsection:   -- imports



// section:      -- imports
// endsection:   -- imports