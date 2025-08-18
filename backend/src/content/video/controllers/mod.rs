use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{
    content::{
        video::{routers::GetVideoQuery, VideoError},
        ListVideo, Video, VideoForCreate,
    },
    DbId,
};

// Stores new video data in db and relevant storage
pub async fn store(db: &Surreal<Client>, video: VideoForCreate) -> Result<DbId, VideoError> {
    let video_db: Option<DbId> = db.create("video").content(video).await?;
    eprintln!("{:?} created", video_db);

    if let Some(video) = video_db {
        Ok(video)
    } else {
        Err(VideoError::FailedToCreate)
    }
}
// Shows list of comics
pub async fn index(db: &Surreal<Client>) -> Result<Vec<ListVideo>, VideoError> {
    let video = db
        .query(format!(
            "SELECT * FROM video FETCH creator, content, content.studio, content.genre;"
        ))
        .await?
        .take(0);
    let video: Vec<ListVideo> = video?;
    return Ok(video);
}

// Shows a specific comic
pub async fn show(db: &Surreal<Client>, query: GetVideoQuery) -> Result<Video, VideoError> {
    if query.video.is_some() {
        let video_id = query.video.unwrap();
        // dbg!(&video_id);
        //let video: Result<Option<video>> = db.select(("video", video_id)).await;

        let video = db
            .query(format!("SELECT * FROM video:{video_id} FETCH creator;"))
            .await?
            .take(0);
        // dbg!(&video);
        match video {
            Ok(video) => {
                if let Some(c) = video {
                    // dbg!(&c);
                    return Ok(c);
                } else {
                    return Err(VideoError::NotFound);
                }
            }
            Err(e) => {
                dbg!(&e);
                return Err(VideoError::DbError(e));
            }
        }
    } else if query.content.is_some() {
        eprintln!("Content");
        let content_id = query.content.unwrap();

        //let content: Option<content> = db.select((content_id[0], content_id[1])).await?;
        let id = format!("{}:{}", "content", content_id);
        let video = db
            .query(format!(
                "SELECT * FROM video WHERE content = {id} FETCH creator;"
            ))
            .await?
            .take(0);
        // dbg!(&video);
        match video {
            Ok(video) => {
                if let Some(vid) = video {
                    // dbg!(&c);
                    return Ok(vid);
                } else {
                    return Err(VideoError::NotFound);
                }
            }
            Err(e) => {
                dbg!(&e);
                return Err(VideoError::DbError(e));
            }
        }
    } else {
        return Err(VideoError::NotFound);
    }
}

// Show form to edit an existing comic
pub async fn edit() {}

// Updates a comic's details
pub async fn update() {}

// Deletes a comic
pub async fn destroy(db: &Surreal<Client>, id: String) -> Result<Video, VideoError> {
    let video_id: Vec<&str> = id.split(':').collect();
    let video: Option<Video> = db.delete((video_id[0], video_id[1])).await?;

    if let Some(vid) = video {
        return Ok(vid);
    } else {
        return Err(VideoError::NotFound);
    }
}
