use surrealdb::{Surreal, engine::remote::ws::Client};
use tracing::debug;

use crate::{content::{video::season::episode::EpisodeError, Episode, EpisodeForCreate, EpisodeForCreateCount, Season}, helpers::db::id_from_thing, Count, DbId, VideoId};


pub async fn store(db: &Surreal<Client>, episode_for_create: EpisodeForCreate) -> Result<DbId, EpisodeError>{
// Select Relative count
    let id = id_from_thing(&episode_for_create.season);
    let query = format!("SELECT count() FROM episode WHERE season = season:{id}");
    let mut relative_count_db = db.query(query).await.unwrap();
    let relative_count_db: Vec<Count> = relative_count_db.take(0)?;
    println!("{:#?}", relative_count_db);
    let relative_count: u32 = if relative_count_db.len() > 0 {
        relative_count_db.len() as u32 + 1
    } else {
        1
    };

    // section: Select Absolute Count

    // Get the video id
    let query = format!("SELECT video FROM season WHERE id = season:{id}");
    let mut video = db.query(query).await.unwrap();
    let video: Option<VideoId> = video.take(0).unwrap();
    let video = video.unwrap();
    let video_id = id_from_thing(&video.video);

    // Select the relevant seasons
    let query = format!("SELECT * FROM season WHERE video = video:{video_id}");
    let mut seasons = db.query(query).await.unwrap();
    // TODO: Test this out as a vec of Id's first, also do this for chapter
    let mut seasons: Vec<Season> = seasons.take(0).unwrap();

    // Sorting the seasons
    seasons.sort_by(|a, b| b.season_no.cmp(&a.season_no));

    let mut absolute_count = 0;
    // Iterate throught the seasons
    for season in seasons {
        let id = id_from_thing(&season.id);

        // Get number of episodes in this season
        let query = format!("SELECT count() FROM episode WHERE season = season:{id}");
        let mut absolute_count_db = db.query(query).await.unwrap();
        let absolute_count_db: Vec<Count> = absolute_count_db.take(0).unwrap();
        println!("{:#?}", absolute_count_db);
        // Since nothing is returned when a season has nothing, zero is assigned to count in such a case.
        let count2: u32 =  if absolute_count_db.len() > 0 {
            absolute_count_db.len() as u32
        } else {
            0
        };

        absolute_count += count2;
    }
    absolute_count += 1;
    eprintln!("Final Count is {absolute_count}");
    // end section: Select Absolute Count

    let new_episode =
        EpisodeForCreateCount::from_epi_create(episode_for_create, relative_count, absolute_count); // Assign both counts to new episode
                                                                                                     // let video = db.select("video", );
    dbg!(&new_episode);
    let episode: Option<DbId> = db.create("episode").content(new_episode).await?;
    eprintln!("Created {:#?}", episode);

    if let Some(c) = episode {
        Ok(c)
    } else {
        Err(EpisodeError::FailedToCreate)
    }
}

pub async fn index(db: &Surreal<Client>, season: String) -> Result<Vec<Episode>, EpisodeError>{
    debug!("{season}");
    let query = format!("SELECT * FROM episode WHERE season = season:{season}"); // TODO: Order this list
    let mut episodes = db.query(query).await?;
    let episodes: Vec<Episode> = episodes.take(0)?;

    return Ok(episodes);
}


pub async fn show(db: &Surreal<Client>) -> Result<DbId, EpisodeError>{
    unimplemented!();
}
pub async fn update(db: &Surreal<Client>) -> Result<DbId, EpisodeError>{
    unimplemented!();
}
pub async fn destroy(db: &Surreal<Client>) -> Result<DbId, EpisodeError>{
    unimplemented!();
}