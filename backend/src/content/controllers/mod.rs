use serde_json::json;
use surrealdb::{engine::remote::ws::Client, Surreal};
use tracing::debug;

use crate::DbId;

use super::{Content, ContentError, ContentForCreateServer, ContentForUpdate};

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

pub async fn get_all(db: &Surreal<Client>) -> Result<Vec<Content>, ContentError> {
    let content_list: Vec<Content> = db.select("content").await?;
    debug!("{:?}", content_list);
    Ok(content_list)
}

pub async fn list_by_studio(db: &Surreal<Client>, id: String) -> Result<Vec<Content>, ContentError>{
    let query = format!("SELECT * FROM content WHERE studio = {id};");
    let mut res = db.query(query).await?;

    let content_list: Vec<Content> = res.take(0)?;
    debug!("{:?}", content_list);
    Ok(content_list)
}


pub async fn update_details(db: &Surreal<Client>,id:String, payload: ContentForUpdate) -> Result<Content, ContentError>{
    let id: &str = id.split(":").collect::<Vec<&str>>()[1];
    //TODO: test if it accepts different types o
    // Items to update username, password, socials, description,
    let mut res: Option<Content> = None;
    let field = payload.field.clone();

    match field.as_str() {
        "title" | "description" | "avatar" => {
            res = db
                .update(("creator", id))
                .merge(json!({&payload.field: &payload.value}))
                .await
                .unwrap(); // reset the number of login attempts
        }
        "password" => {
            // Password is also special
            println!("IS IT THE BRAIDS") //TODO: Implement this
        }

        &_ => return Err(ContentError::FailedToCreate), //Change to update error
    }

    if res.is_none() {
        Err(ContentError::DetailsUpdateError)
    } else if let Some(c) = res {
        println!("{:?}", c);
        Ok(c)
    } else {
        Err(ContentError::DetailsUpdateError)
    }
}