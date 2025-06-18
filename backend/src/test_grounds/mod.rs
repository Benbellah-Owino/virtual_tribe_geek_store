use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use http::StatusCode;

use crate::{
    content::{comic::volume::chapter::ChapterForCreate, Volume}, dev_initial::db::Db, helpers::db::id_from_thing, ComicId, Count, DbId
};

pub fn test_grounds_router() -> Router<Db> {
    return Router::new().route("/test_api", post(test_fn));
}

async fn test_fn(
    State(dbt): State<Db>,
    Json(payload): Json<ChapterForCreate>,
) -> impl IntoResponse {
    let db = &dbt.unwrap();
    let id = id_from_thing(&payload.volume);
    eprintln!("Id -> {id}");

    // Get the comic id
    let query = format!("SELECT comic FROM volume WHERE id = volume:{id}");
    let mut comic = db.query(query).await.unwrap();
    let comic:Option<ComicId> = comic.take(0).unwrap();
    let comic = comic.unwrap();
    let comic_id = id_from_thing(&comic.comic);

    // Select the relevant volumes
    let query = format!("SELECT * FROM volume WHERE comic = comic:{comic_id}");
    let mut volumes = db.query(query).await.unwrap();
    let mut volumes: Vec<Volume> = volumes.take(0).unwrap();

    // Sorting the volumes
    volumes.sort_by(|a, b| b.vol_no.cmp(&a.vol_no));

    let mut a_count = 0;
    // Iterate throught the volumes
    for volume in volumes {
        let id = id_from_thing(&volume.id);
        
        // Get number of chapters in this volume
        let query = format!("SELECT count() FROM chapter WHERE volume = volume:{id}"); 
        let mut count = db.query(query).await.unwrap();
        let count: Vec<Count> = count.take(0).unwrap();
        println!("{:#?}", count);
        // Since nothing is returned when a volume has nothing, zero is assigned to count in such a case.
        let mut count2: u32 = 0;
        if count.len() > 0 {
            count2 = count.len() as u32;
        } else {
            count2 = 0;
        };
        
        a_count += count2;
    }
    a_count += 1;
    eprintln!("Final Count is {a_count}");
    return (StatusCode::OK, id);
}
