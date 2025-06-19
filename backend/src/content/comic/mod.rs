use serde::Serialize;

pub mod controllers;
pub mod routers;

pub mod volume;

// region:      --- Structs
// endregion:   --- Structs

// region:      --- Error
#[derive(Debug, Serialize)]
pub enum ComicError {
    NotFound,
    FailedToCreate,
    RetrievalError,
    DetailsUpdateError,
    DbError(surrealdb::Error),
}

impl From<surrealdb::Error> for ComicError {
    fn from(value: surrealdb::Error) -> Self {
        ComicError::DbError(value)
    }
}
// endregion:   --- Error

// region:      ---
// endregion:   ---
