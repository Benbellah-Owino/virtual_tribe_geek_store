use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

pub mod chapter;
pub mod controllers;
pub mod routers;

// region:      --- Structs

// endregion:   --- Structs

// region:      --- Error
#[derive(Debug, Serialize)]
pub enum VolumeError {
    NotFound,
    FailedToCreate,
    RetrievalError,
    DetailsUpdateError,
    DbError(surrealdb::Error),
}

impl From<surrealdb::Error> for VolumeError {
    fn from(value: surrealdb::Error) -> Self {
        VolumeError::DbError(value)
    }
}
// endregion:   --- Error

// TODO:-> Some changes were made, make sure to test this module