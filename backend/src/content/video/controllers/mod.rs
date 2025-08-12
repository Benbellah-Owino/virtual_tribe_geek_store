use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{content::{video::VideoError, VideoForCreate}, DbId};

// Stores new video data in db and relevant storage
pub async fn store(db: &Surreal<Client>, video: VideoForCreate) -> Result<DbId, VideoError>{
    let video_db: Option<DbId> = db.create("video").content(video).await?;
    eprintln!("{:?} created", video_db);

    if let Some(video) = video_db{
        Ok(video)
    }else{
        Err(VideoError::FailedToCreate)
    }

}
// Shows list of comics
pub async fn index(){}

// Shows a specific comic
pub async fn show(){}


// Show form to edit an existing comic
pub async fn edit(){}

// Updates a comic's details
pub async fn update(){}

// Deletes a comic
pub async fn destroy(){}