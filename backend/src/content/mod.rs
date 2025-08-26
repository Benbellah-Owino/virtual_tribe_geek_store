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
    pub cover: Option<String>,
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
    pub cover: Option<String>,
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
        let genre = content.genre.into_iter().map(thing_from_string).collect();
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
    value: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Comic {
    pub id: Thing,
    pub volumes: u16,
    pub chapters: u16,
    pub writer: Vec<String>,
    pub artist: Vec<String>,
    pub creator: Vec<Creator>, // Make it so that it fetches the creator details
    pub created_at: String,
    pub cover: Option<String>,
    pub content: Thing,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListComic {
    pub id: Thing,
    pub volumes: u16,
    pub chapters: u16,
    pub writer: Vec<String>,
    pub artist: Vec<String>,
    pub creator: Vec<Creator>, // Make it so that it fetches the creator details
    pub created_at: String,
    pub cover: Option<String>,
    pub content: Content,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComicForCreate {
    pub writer: Vec<String>,
    pub artist: Vec<String>,
    pub creator: Vec<Thing>,
    pub content: Thing,
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
    pub no_of_chapters: i16,
    pub runlength: Option<Runlength>,
    pub synopsis: Option<String>,
    pub comic: Thing,
    pub cover: Option<String>,
    pub vol_no: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VolumeForCreate {
    pub synopsis: String,
    pub comic: Thing,
    pub cover: Option<String>,
    pub no_of_chapters: i16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VolumeForCreateCount {
    pub synopsis: String,
    pub comic: Thing,
    pub cover: Option<String>,
    pub no_of_chapters: i16,
    pub vol_no: u32,
}

impl VolumeForCreateCount {
    fn from_vol_create(vol: VolumeForCreate, count: u32) -> Self {
        VolumeForCreateCount {
            vol_no: count,
            synopsis: vol.synopsis,
            comic: vol.comic,
            cover: vol.cover,
            no_of_chapters: vol.no_of_chapters,
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Runlength {
    pub start: Thing,
    pub end: Thing,
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
    pub writer: Vec<String>,
    pub creator: Vec<Creator>, // Make it so that it fetches the creator details
    pub cover: Option<String>,
    pub trailer: Option<String>,
    pub video_type: Option<String>, // Series or Movie
    pub average_run_length: EpiRunLength,
    pub created_at: String,
    pub content: Thing,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListVideo {
    pub id: Thing,
    pub seasons: u16,
    pub episodes: u16,
    pub writer: Vec<String>,
    pub creator: Vec<Creator>, // Make it so that it fetches the creator details
    pub cover: Option<String>,
    pub trailer: Option<String>,
    pub video_type: Option<String>, // Series or Movie
    pub average_run_length: EpiRunLength,
    pub created_at: String,
    pub content: Content,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VideoForCreate {
    pub writer: Vec<String>,
    pub creator: Vec<Thing>, // Make it so that it fetches the creator details
    pub video_type: Option<String>, // Series or Movie
    pub average_run_length: EpiRunLength,
    pub content: Thing,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EpiRunLength {
    pub seconds: i32,
    pub minutes: i32,
    pub hours: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SeasonRunlength {
    pub id: Thing,
    start: Thing,
    end: Thing,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SkipRunLength {
    pub start: EpiRunLength,
    pub end: EpiRunLength,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Season {
    pub id: Thing,
    pub no_of_episodes: u16,
    pub episodes_available: u16,
    pub season_no: u16,
    pub runlength: Option<SeasonRunlength>,
    pub synopsis: String,
    pub video: Thing,
    pub cover: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SeasonForCreate {
    pub no_of_episodes: u16,
    pub synopsis: String,
    pub video: Thing,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SeasonForCreateCount {
    pub no_of_episodes: u16,
    pub synopsis: String,
    pub video: Thing,
    pub season_no: u32,
}

impl SeasonForCreateCount {
    fn from_sn_create(season: SeasonForCreate, count: u32) -> Self {
        SeasonForCreateCount {
            no_of_episodes: season.no_of_episodes,
            synopsis: season.synopsis,
            video: season.video,
            season_no: count,
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Episode {
    pub title: String,
    pub relative_episode: u32, // Relative to the season
    pub absolute_episode: u32, // Numbering irregardless of season
    pub synopsis: String,
    pub file: Option<String>,
    pub cover: Option<String>,
    pub season: Thing,
    pub runlength: Option<EpiRunLength>, // Default will be the one specified by the user
    pub opening_length: SkipRunLength,
    pub closing_length: SkipRunLength,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EpisodeForCreate {
    pub title: String,
    pub runlength: Option<EpiRunLength>,
    pub opening_length: SkipRunLength,
    pub closing_length: SkipRunLength,
    pub synopsis: String,
    pub season: Thing,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EpisodeForCreateCount {
    pub title: String,
    pub runlength: Option<EpiRunLength>,
    pub opening_length: SkipRunLength,
    pub closing_length: SkipRunLength,
    pub synopsis: String,
    pub season: Thing,
    pub relative_episode: u32, // Relative to the season
    pub absolute_episode: u32, // Numbering irregardless of season
}


impl EpisodeForCreateCount {
    fn from_epi_create(epi: EpisodeForCreate,  relative_count: u32, absolute_count: u32) -> Self {
        EpisodeForCreateCount {
            title: epi.title,
            runlength: epi.runlength,
            synopsis: epi.synopsis,
            season: epi.season,
            absolute_episode: absolute_count,
            relative_episode: relative_count,
            opening_length: epi.opening_length,
            closing_length: epi.closing_length,
        }
    }
}



#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EpisodeRunlength {
    pub id: Thing,
    start: Runlength,
    end: Runlength,
}

// section error
#[derive(Debug, Serialize)]
pub enum ContentError {
    NotFound,
    FailedToCreate,
    RetrievalError,
    DetailsUpdateError,
    InvalidFieldError,
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
