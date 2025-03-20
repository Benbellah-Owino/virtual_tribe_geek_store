use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;

use crate::{content::{Comic, ComicForCreate, ContentForUpdate}, DbId};

use super::ComicError;


// Shows list of comics
pub async fn index(db: &Surreal<Client>)-> Result<Vec<Comic>, ComicError>{
    let comic: Vec<Comic> = db.select("comic").await?;

    return Ok(comic)
}

// Shows a specific comic
pub async fn show(db: &Surreal<Client>, id: String) -> Result<Comic, ComicError>{
    //TODO: Use a custom Select query to accomodate Joins
    let comic_id: Vec<&str> = id.split(':').collect();
    let comic: Option<Comic> = db.select((comic_id[0],comic_id[1])).await?;

    if let Some(c) = comic{
        return Ok(c)
    }else{
        return Err(ComicError::NotFound)
    }
}

// Stores new comic data in db and relevant storage
pub async fn store(db: &Surreal<Client>, comic: ComicForCreate) -> Result<DbId, ComicError>{
    let comic_db: Option<DbId> = db.create("comic").content(comic).await?;
    eprintln!("{:?} created", comic_db);

    if let Some(c) = comic_db{
        Ok(c)
    }else{
        Err(ComicError::FailedToCreate)
    }
}

// Updates a comic's details
pub async fn update(db: &Surreal<Client>, id: String, payload: ContentForUpdate) -> Result<Comic, ComicError>{
    unimplemented!()
}

// Deletes a comic
pub async fn destroy(db: &Surreal<Client>, id: String) -> Result<Comic, ComicError>{
    let comic_id: Vec<&str> = id.split(':').collect(); 
    let comic: Option<Comic> = db.delete((comic_id[0],comic_id[1])).await?;

    if let Some(c) = comic{
        return Ok(c)
    }else{
        return Err(ComicError::NotFound)
    }
}