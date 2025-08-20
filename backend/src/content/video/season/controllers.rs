use surrealdb::{engine::remote::ws::Client, Surreal};
use tracing::debug;

use crate::{content::{video::season::SeasonError, Season, SeasonForCreate}, DbId};

// Shows list of seasons
pub async fn index(db: &Surreal<Client>, video: String) -> Result<Vec<Season>, SeasonError>{
    debug!("{video}");
    let query = format!("SELECT * FROM season WHERE video = video:{video}"); // TODO: Order this list
    let mut seasons = db.query(query).await?;
    let seasons: Vec<Season> = seasons.take(0)?;
    // vort_by(|a,b|
    //     b.vol_no.cmp(&a.vol_no)
    // );
    return Ok(seasons);
}

// Shows a specific season
pub async fn show(db: &Surreal<Client>, id: String) -> Result<Season, SeasonError>{

    let season: Option<Season> = db.select(("season", id)).await?;

    if let Some(sn) = season {
        return Ok(sn);
    } else {
        return Err(SeasonError::NotFound);
    }
}


// Stores new season data in db and relevant storage
pub async fn store(db: &Surreal<Client>, season: SeasonForCreate) -> Result<DbId, SeasonError>{
    let season_db: Option<DbId> = db.create("season").content(season).await?;
    eprintln!("{:?} created", season_db);

    if let Some(season) = season_db {
        Ok(season)
    } else {
        Err(SeasonError::FailedToCreate)
    }
}


// Show form to edit an existing season
pub async fn edit(){}

// Updates a season's details
pub async fn update(){}

// Deletes a season
pub async fn destroy(){}