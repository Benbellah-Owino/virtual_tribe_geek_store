// region:      --- Imports
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::DbId;

use super::{Genre, GenreError, GenreForCreate};
// endregion:   --- Imports

// region:      --- Controllers
pub async fn store(genre: GenreForCreate, db: &Surreal<Client>) -> Result<DbId, GenreError> {
    let new_genre: Option<DbId> = db.create("genre").content(genre).await?;
    if let Some(g) =  new_genre{
        Ok(g)
    }else{
        Err(GenreError::RetrievalError)
    }
}

pub async fn list(db: &Surreal<Client>) -> Result<Vec<Genre>, GenreError> {
    let genre_list: Vec<Genre> = db.select("genre").await?;
    Ok(genre_list)
}
// endregion:   --- Controllers
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
