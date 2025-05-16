use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::{creator::Creator, helpers::db::thing_from_string, studio::StudioFull};

mod controllers;
pub mod routers;

pub mod comic;
pub mod genre;
pub mod video;



// region:      --- Types
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Genre {
    pub id: Thing,
    pub name: String,
    pub description: String,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContentList {
    pub id: Thing,
    pub title: String,
    pub rating: f32,
    pub recom_price: f32,

}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Content {
    pub id: Thing,
    pub title: String,
    pub rating: f32,
    pub studio: StudioFull,
    pub description: String,
    pub audiences: String,
    pub recom_price: f32,
    pub genre: Vec<Genre>,
    pub cover: Option<String>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContentUpdated {
    //TODO Find a way to return genre
    pub id: Thing,
    pub title: String,
    pub rating: f32,
    pub studio: StudioFull,
    pub description: String,
    pub audiences: String,
    pub recom_price: f32,
    pub cover: Option<String>
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContentForCreateClient {
    pub title: String,
    pub studio: String,
    pub description: String,
    pub audiences: String,
    pub recom_price: f32,
    pub genre: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContentForCreateServer {
    pub title: String,
    pub studio: Thing,
    pub description: String,
    pub audiences: String,
    pub recom_price: f32,
    pub genre: Vec<Thing>,
}

impl From<ContentForCreateClient> for ContentForCreateServer {
    fn from(content: ContentForCreateClient) -> Self {
        let genre = content
            .genre
            .into_iter()
            .map(thing_from_string)
            .collect();
        ContentForCreateServer {
            studio: thing_from_string(content.studio),
            genre,
            title: content.title,
            description: content.description,
            audiences: content.audiences,
            recom_price: content.recom_price,
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContentForUpdate {
    field: String,
    value: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Comic {
    pub id: Thing,
    pub volumes: u16,
    pub chapters: u16,
    pub writer: Vec<String>,
    pub artist: Vec<String>,
    pub creator: Vec<Creator>,// Make it so that it fetches the creator details
    pub created_at: String,
    pub cover: Option<String>,
    pub content: Thing
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComicForCreate {
    pub writer: Vec<String>,
    pub artist: Vec<String>,
    pub creator: Vec<Thing>,
    pub content: Thing
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComicRunlength {
    pub id: Thing,
    start: Option<Thing>,
    end: Option<Thing>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Volume {
    pub id: Thing,
    pub no_of_chapters: u16,
    pub runlength: ComicRunlength,
    pub synopsis: String,
    pub comic: Thing,
    pub cover: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Chapter {
    pub id: Thing,
    pub relative_chapter: u32,
    pub absolute_chapter: u32,
    pub pages: u16,
    pub sypnosis: String,
    pub file: Option<String>,
    pub cover: Option<String>,
    pub volume: Thing,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Video {
    pub id: Thing,
    pub seasons: u16,
    pub episodes: u16,
    pub writer: Vec<Thing>,
    pub animation: Vec<Thing>,
    pub average_run_length: f32,
    pub category: String,
    pub created_at: String,
    pub cover: Option<String>,
    pub trailer: Option<String>,
    pub video_type: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EpisodeRunlength {
    pub id: Thing,
    start: f32,
    end: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SeasonRunlength {
    pub id: Thing,
    start: Thing,
    end: Thing,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Season {
    pub id: Thing,
    pub no_of_episodes: u16,
    pub runlength: SeasonRunlength,
    pub synopsis: String,
    pub video: Thing,
    pub cover: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Episode {
    pub relative_episode: u32, // Relative to the season
    pub absolute_episode: u32, // Numbering irregardless of season
    pub runlength: EpisodeRunlength,
    pub skiplength: EpisodeRunlength,
    pub sypnosis: String,
    pub file: Option<String>,
    pub cover: Option<String>,
    pub season: Thing,
}

// section error
#[derive(Debug, Serialize)]
pub enum ContentError {
    NotFound,
    FailedToCreate,
    RetrievalError,
    DetailsUpdateError,
    DbError(surrealdb::Error),
}

impl From<surrealdb::Error> for ContentError {
    fn from(value: surrealdb::Error) -> Self {
        ContentError::DbError(value)
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
// region:      ---
// endregion:   ---
// region:      ---
// endregion:   ---
