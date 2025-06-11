use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

pub mod routers;
pub mod controllers;



// region:      --- Structs
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Chapter{
    pub id: Thing,
    pub relative_chapter: u32,
    pub absolute_chapter: u32,
    pub pages: u16,
    pub synopsis: Option<String>,
    pub file: Option<String>,
    pub volume: Thing
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChapterForCreate{
    pub id: Thing,
    pub pages: u16,
    pub synopsis: Option<String>,
    pub file: Option<String>,
    pub volume: Thing
}
// endregion:   --- Structs


// region:      --- Error

#[derive(Debug, Serialize)]
pub enum ChapterError {
    NotFound,
    FailedToCreate,
    RetrievalError,
    DetailsUpdateError,
    DbError(surrealdb::Error),
}

impl From<surrealdb::Error> for ChapterError {
    fn from(value: surrealdb::Error) -> Self {
        ChapterError::DbError(value)
    }
}
// endregion:   --- Error