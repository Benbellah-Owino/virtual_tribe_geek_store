use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

pub mod controllers;
pub mod routers;

// region:      --- Structs
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Chapter {
    pub id: Thing,
    pub title: String,
    pub relative_chapter: u32,
    pub absolute_chapter: u32,
    pub pages: u16,
    pub synopsis: Option<String>,
    pub file: Option<String>,
    pub cover: Option<String>,
    pub volume: Thing,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChapterForCreate {
    pub pages: u16,
    pub title: String,
    pub synopsis: Option<String>,
    pub volume: Thing,
    pub cover: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChapterForCreateCount {
    pub pages: u16,
    pub title: String,
    pub synopsis: Option<String>,
    pub volume: Thing,
    pub relative_chapter: u32,
    pub absolute_chapter: u32,
    pub cover: Option<String>,
}

impl ChapterForCreateCount {
    fn from_chap_create(
        chap: ChapterForCreate,
        relative_count: u32,
        absolute_count: u32,
    ) -> ChapterForCreateCount {
        ChapterForCreateCount {
            pages: chap.pages,
            synopsis: chap.synopsis,
            volume: chap.volume,
            relative_chapter: relative_count,
            absolute_chapter: absolute_count,
            title: chap.title,
            cover: chap.cover,
        }
    }
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
