use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use tracing::debug;
use crate::helpers::db::id_from_thing;
use crate::DbId;
use crate::content::comic::volume::chapter::{Chapter,ChapterForCreate,ChapterError};

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
    let id = id_from_thing(chapter_for_create.volume);
    let query = format!("SELECT count() FROM chapter WHERE volume = volume:{id}");
    let mut count =  db.query(query).await.unwrap();
    let count:Vec<Count>  = count.take(0)?;
    println!("{:#?}", count);
    let mut count2:u32 =0; 
    if count.len() > 0{
        let ct = count.len() + 1;
        eprintln!("{}",ct);
        count2 =  count.len() as u32 + 1;
    }else{
        count2 = 1;
    };
    
    // TODO: Select Absolute Count

    let chapter: Option<DbId> = db.create("chapter").content(chapter_for_create).await?;
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
