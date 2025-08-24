// region:      --- Modules

use serde::Serialize;

pub mod controllers;
pub mod routers;


// endregion:   --- Modules
// region:      --- Enums
#[derive(Debug, Serialize)]
pub enum EpisodeError {
    NotFound,
    FailedToCreate,
    RetrievalError,
    DetailsUpdateError,
    InvalidFieldError,
    DbError(surrealdb::Error),
}

impl From<surrealdb::Error> for EpisodeError {
    fn from(value: surrealdb::Error) -> Self {
        EpisodeError::DbError(value)
    }
}
// endregion:   --- Enums
