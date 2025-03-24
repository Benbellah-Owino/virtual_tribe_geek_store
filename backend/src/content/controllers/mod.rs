use serde_json::json;
use surrealdb::{engine::remote::ws::Client, Surreal};
use tracing::debug;

use crate::DbId;

use super::{Content, ContentError, ContentForCreateServer, ContentForUpdate, ContentList, ContentUpdated};

pub async fn store(
    content: ContentForCreateServer,
    db: &Surreal<Client>,
) -> Result<DbId, ContentError> {
    let content_db: Option<DbId> = db.create("content").content(content).await?;
    eprintln!("{:?} created", content_db);
    if let Some(c) =  content_db{
        Ok(c)
    }else{
        Err(ContentError::RetrievalError)
    }

}


pub async fn get_content(db: &Surreal<Client>, id: String) -> Result<Content, ContentError>{
    //let id: &str = id.split(":").collect::<Vec<&str>>()[1];
    //let content: Option<Content> = db.select(("content", id)).await?; 
    debug!("Getting content from db");
    let query = format!("SELECT * FROM content WHERE id = {id} FETCH genre, studio");
    debug!("{query}");
    let mut res = db.query(&query).await?;

    let content: Option<Content> = res.take(0)?;
    
    eprintln!("Content {:?}", &content);
    eprintln!("Retrieved");
    if let Some(c) = content{
        dbg!(&c);
        return Ok(c);
    }else{
        return Err(ContentError::NotFound);
    }
}


pub async fn get_all(db: &Surreal<Client>) -> Result<Vec<Content>, ContentError> {
    let content_list: Vec<Content> = db.select("content").await?;
    debug!("{:?}", content_list);
    Ok(content_list)
}

pub async fn list_by_studio(db: &Surreal<Client>, id: String) -> Result<Vec<ContentList>, ContentError>{
    let query = format!("SELECT * FROM content WHERE studio = {id} FETCH genre;");
    let mut res = db.query(query).await?;
    dbg!(&res);
    debug!("Goten content list");
    let content_list: Vec<ContentList> = res.take(0)?;
    eprintln!("{:?}", content_list);
    Ok(content_list)
}


pub async fn update_details(db: &Surreal<Client>,id:&str, payload: ContentForUpdate) -> Result<DbId, ContentError>{
    //TODO: test if it accepts different types o
    // Items to update username, password, socials, description,
    let mut res: Option<DbId> = None;
    //debug!("{:#?}",&payload);
    let field = payload.field.clone();
    match field.as_str() {
        "title" | "description" | "cover" => {
            res = db
                .update(("content", id))
                .merge(json!({&payload.field: &payload.value}))
                .await
                .map_err(|e| dbg!(e))?; // reset the number of login attempts
        }

        &_ => return Err(ContentError::FailedToCreate), //Change to update error
    }

    if res.is_none() {
        Err(ContentError::DetailsUpdateError)
    } else if let Some(c) = res {
        debug!("{:?}", c);
        Ok(c)
    } else {
        Err(ContentError::DetailsUpdateError)
    }
}