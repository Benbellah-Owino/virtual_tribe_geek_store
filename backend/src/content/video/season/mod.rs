use serde::Serialize;

pub mod controllers;
pub mod routers;



#[derive(Debug, Serialize)]
pub enum SeasonError {
    NotFound,
    FailedToCreate,
    RetrievalError,
    DetailsUpdateError,
    InvalidFieldError,
    DbError(surrealdb::Error),
}

impl From<surrealdb::Error> for SeasonError {
    fn from(value: surrealdb::Error) -> Self {
        SeasonError::DbError(value)
    }
}