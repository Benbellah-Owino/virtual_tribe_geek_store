// region:      --- Imports
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
// endregion:   --- Imports

// region:      --- SubModules
mod controllers;
pub mod routers;
// endregion:   --- SubModules

// region:      --- Types
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Genre {
    id: Thing,
    name: String,
    description: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GenreForCreate {
    name: String,
    description: String,
}

#[derive(Debug, Serialize)]
pub enum GenreError {
    FailedToCreate,
    DbError(surrealdb::Error),
}

impl From<surrealdb::Error> for GenreError {
    fn from(value: surrealdb::Error) -> Self {
        GenreError::DbError(value)
    }
}

// endregion:   --- Types

// region:      ---
// endregion:   ---
// region:      ---
// endregion:   ---
// region:      ---
// endregion:   ---
// region:      ---
// endregion:   ---
