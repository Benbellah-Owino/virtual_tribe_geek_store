use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

use crate::{
    content::{Comic, ComicForCreate, ContentForUpdate, ListComic},
    DbId,
};

use super::{routers::GetComicQuery, ComicError};

// Shows list of comics
pub async fn index(db: &Surreal<Client>) -> Result<Vec<ListComic>, ComicError> {
    let comic= db
            .query(format!("SELECT * FROM comic FETCH creator, content, content.studio, content.genre;"))
            .await?
            .take(0);
    let comic: Vec<ListComic> = comic?;
    return Ok(comic);
}

// Shows a specific comic
pub async fn show(db: &Surreal<Client>, query: GetComicQuery) -> Result<Comic, ComicError> {
    //TODO: Use a custom Select query to accomodate Joins
    if query.comic.is_some() {
        let comic_id = query.comic.unwrap();
        // dbg!(&comic_id);
        //let comic: Result<Option<Comic>> = db.select(("comic", comic_id)).await;

        let comic = db
            .query(format!("SELECT * FROM comic:{comic_id} FETCH creator;"))
            .await?
            .take(0);
        // dbg!(&comic);
        match comic {
            Ok(comic) => {
                if let Some(c) = comic {
                    // dbg!(&c);
                    return Ok(c);
                } else {
                    return Err(ComicError::NotFound);
                }
            }
            Err(e) => {
                dbg!(&e);
                return Err(ComicError::DbError(e));
            }
        }
    } else if query.content.is_some() {
        eprintln!("Content");
        let content_id = query.content.unwrap();

        //let content: Option<content> = db.select((content_id[0], content_id[1])).await?;
        let id = format!("{}:{}", "content", content_id);
        let comic = db
            .query(format!(
                "SELECT * FROM comic WHERE content = {id} FETCH creator;"
            ))
            .await?
            .take(0);
        // dbg!(&comic);
        match comic {
            Ok(comic) => {
                if let Some(c) = comic {
                    // dbg!(&c);
                    return Ok(c);
                } else {
                    return Err(ComicError::NotFound);
                }
            }
            Err(e) => {
                dbg!(&e);
                return Err(ComicError::DbError(e));
            }
        }
    } else {
        return Err(ComicError::NotFound);
    }
}

// Stores new comic data in db and relevant storage
pub async fn store(db: &Surreal<Client>, comic: ComicForCreate) -> Result<DbId, ComicError> {
    let comic_db: Option<DbId> = db.create("comic").content(comic).await?;
    eprintln!("{:?} created", comic_db);

    if let Some(c) = comic_db {
        Ok(c)
    } else {
        Err(ComicError::FailedToCreate)
    }
}

// Updates a comic's details
pub async fn update(
    db: &Surreal<Client>,
    id: String,
    payload: ContentForUpdate,
) -> Result<Comic, ComicError> {
    unimplemented!()
}

// Deletes a comic
pub async fn destroy(db: &Surreal<Client>, id: String) -> Result<Comic, ComicError> {
    let comic_id: Vec<&str> = id.split(':').collect();
    let comic: Option<Comic> = db.delete((comic_id[0], comic_id[1])).await?;

    if let Some(c) = comic {
        return Ok(c);
    } else {
        return Err(ComicError::NotFound);
    }
}
