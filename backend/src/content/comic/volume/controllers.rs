use serde::{Serialize,Deserialize};
use surrealdb::{engine::remote::ws::Client, Surreal};
use tracing::debug;
use crate::{content::comic::volume::VolumeForCreateCount, DbId};

use super::{Volume, VolumeError, VolumeForCreate};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Count{
    pub count: u32 
}

// Shows list of volumes
pub async fn index(db: &Surreal<Client>, comic: String) -> Result<Vec<Volume>, VolumeError>{
    debug!("{comic}");
    let query = format!("SELECT * FROM volume WHERE comic = comic:{comic}");
    debug!("QUERY -> {query}");
    let mut volumes = db.query(query).await?;
    let volumes: Vec<Volume> = volumes.take(0)?;
    dbg!(&volumes);
    return Ok(volumes)
}

// Shows a specific volume
pub async fn show(db: &Surreal<Client>, id: String)-> Result<Volume, VolumeError>{
    // let id: &str = id.split(":").collect::<Vec<&str>>()[1];
    let volume: Option<Volume> = db.select(("volume", id)).await?;

    if let Some(v) = volume{
        return Ok(v)
    }else{
        return Err(VolumeError::NotFound)
    }
}


// Stores new volume data in db and relevant storage
pub async fn store(db: &Surreal<Client>, mut volume_for_create: VolumeForCreate) -> Result<DbId, VolumeError>{
   /*  let mut count =  db.query("SELECT count() FROM volume").await.unwrap();
    let count: Option<i32> = count.take(0)? */;
    let mut count =  db.query("SELECT count() FROM volume").await.unwrap();
    println!("{:#?}", count);
    let count:Vec<Count>  = count.take(0)?;
    println!("{:#?}", count);
    let count = count[1].count + 1;

    let vol = VolumeForCreateCount::from_vol_create(volume_for_create, count);
    let volume: Option<DbId> = db.create("volume").content(vol).await?;
    eprintln!("Created {:#?}", volume);

    if let Some(v) =  volume{
        Ok(v)
    }else{
        Err(VolumeError::RetrievalError)
    }

}

// Show form to edit an existing volume
pub async fn edit(id: String){
}

// Updates a volume's details
pub async fn update(id: String){
}

// Deletes a volume
pub async fn destroy(id: String){
}
