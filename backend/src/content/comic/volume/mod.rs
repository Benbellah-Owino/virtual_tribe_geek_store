use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

pub mod routers;
pub mod controllers;



// region:      --- Structs
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Volume{
    pub id: Thing,
    pub no_of_chapters: i16, 
    pub runlength: Option<Runlength>,
    pub synopsis: Option<String>,
    pub comic: Thing,
    pub cover: Option<String>,
    pub vol_no: u32
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VolumeForCreate{
    pub synopsis: String,
    pub comic: Thing,
    pub cover: Option<String>,
    pub no_of_chapters: i16, 
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VolumeForCreateCount{
    pub synopsis: String,
    pub comic: Thing,
    pub cover: Option<String>,
    pub no_of_chapters: i16, 
    pub vol_no: u32
}

impl VolumeForCreateCount{
    fn from_vol_create(vol: VolumeForCreate, count: u32) -> Self {
        VolumeForCreateCount{
            vol_no: count,
            synopsis: vol.synopsis,
            comic: vol.comic,
            cover: vol.cover,
            no_of_chapters: vol.no_of_chapters,
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Runlength{
    pub start: Thing,
    pub end: Thing,
}
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