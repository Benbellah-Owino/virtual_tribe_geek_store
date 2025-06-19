use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use tracing::debug;
use crate::content::comic::volume::controllers::show as volume_show;
use crate::content::comic::volume::Volume;
use crate::helpers::db::id_from_thing;
use crate::{ComicId, Count, DbId};
use crate::content::comic::volume::chapter::{Chapter, ChapterError, ChapterForCreate, ChapterForCreateCount};

// Shows list of chapters
pub async fn index(db: &Surreal<Client>, volume: String) -> Result<Vec<Chapter>, ChapterError>{
    debug!("{volume}");
    let query = format!("SELECT * FROM chapter WHERE volume = volume:{volume}"); // TODO: Order this list
    debug!("QUERY -> {query}");
    let mut chapters = db.query(query).await?;
    let mut chapters: Vec<Chapter> = chapters.take(0)?;
    // chapters.sort_by(|a,b| 
    //     b.vol_no.cmp(&a.vol_no)
    // );
    return Ok(chapters)
}


// Shows a specific chapter
pub async fn show(db: &Surreal<Client>, id: String)-> Result<Chapter, ChapterError>{
    let chapter: Option<Chapter> = db.select(("chapter", id)).await?;

    if let Some(c) = chapter{
        Ok(c)
    }else{
        Err(ChapterError::FailedToCreate)
    }
}


// Stores new chapter data in db and relevant storage
pub async fn store(db: &Surreal<Client>, chapter_for_create: ChapterForCreate) -> Result<DbId, ChapterError>{
    
    // Select Relative count
    let id = id_from_thing(&chapter_for_create.volume);
    let query = format!("SELECT count() FROM chapter WHERE volume = volume:{id}");
    let mut relative_count_db =  db.query(query).await.unwrap();
    let relative_count_db:Vec<Count>  = relative_count_db.take(0)?;
    println!("{:#?}", relative_count_db);
    let mut relative_count:u32 =0; 
    if relative_count_db.len() > 0{
        relative_count =  relative_count_db.len() as u32 + 1;
    }else{
        relative_count = 1;
    };
    
    // section: Select Absolute Count

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

    let mut absolute_count = 0;
    // Iterate throught the volumes
    for volume in volumes {
        let id = id_from_thing(&volume.id);
        
        // Get number of chapters in this volume
        let query = format!("SELECT count() FROM chapter WHERE volume = volume:{id}"); 
        let mut absolute_count_db = db.query(query).await.unwrap();
        let absolute_count_db: Vec<Count> = absolute_count_db.take(0).unwrap();
        println!("{:#?}", absolute_count_db);
        // Since nothing is returned when a volume has nothing, zero is assigned to count in such a case.
        let mut count2: u32 = 0;
        if absolute_count_db.len() > 0 {
            count2 = absolute_count_db.len() as u32;
        } else {
            count2 = 0;
        };
        
        absolute_count += count2;
    }
    absolute_count += 1;
    eprintln!("Final Count is {absolute_count}");
    // end section: Select Absolute Count

    let new_chapter = ChapterForCreateCount::from_chap_create(chapter_for_create, relative_count, absolute_count); // Assign both counts to new chapter
    // let comic = db.select("comic", );

    let chapter: Option<DbId> = db.create("chapter").content(new_chapter).await?;
    eprintln!("Created {:#?}", chapter);

    if let Some(c) = chapter{
        Ok(c)
    }else{
        Err(ChapterError::FailedToCreate)
    }
}


// Show form to edit an existing chapter
pub async fn edit(id: String){
}


// Updates a chapter's details
pub async fn update(id: String){
}


// Deletes a chapter
pub async fn destroy(id: String){
}
