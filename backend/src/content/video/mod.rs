use serde::Serialize;

pub mod controllers;
pub mod routers;


#[derive(Debug, Serialize)]
pub enum VideoError {
    NotFound,
    FailedToCreate,
    RetrievalError,
    DetailsUpdateError,
    DbError(surrealdb::Error),
}

impl From<surrealdb::Error> for VideoError {
    fn from(value: surrealdb::Error) -> Self {
        VideoError::DbError(value)
    }
}