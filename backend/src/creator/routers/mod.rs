// section:      -- imports
use axum::{
    Json,
    Router,
    routing::*,
};
use axum::response::IntoResponse;
use axum::extract::State;
use axum::http::StatusCode;
use surrealdb::Surreal;

use crate::dev_initial::db::Db;

use super::controllers::register;
use super::CreatorForCreate;
// endsection:   -- imports



// section:      -- router
pub fn creator_router() -> Router<Db>{
    return Router::new()
        .route("/", post(register_handler))
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
// endsection:   -- handlers



// section:      -- imports
// endsection:   -- imports



// section:      -- imports
// endsection:   -- imports