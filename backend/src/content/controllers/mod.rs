use surrealdb::{engine::remote::ws::Client, Surreal};
use tracing::debug;

use crate::DbId;

use super::{Content, ContentError, ContentForCreateServer};

pub async fn store(
    content: ContentForCreateServer,
    db: &Surreal<Client>,
) -> Result<DbId, ContentError> {
    let content_db: Vec<DbId> = db.create("content").content(content).await?;
    eprintln!("{:?} created", content_db);
    return Ok(content_db[0].clone());
}

pub async fn get_all(db: &Surreal<Client>) -> Result<Vec<Content>, ContentError> {
    let content_list: Vec<Content> = db.select("content").await?;
    debug!("{:?}", content_list);
    return Ok(content_list);
}
