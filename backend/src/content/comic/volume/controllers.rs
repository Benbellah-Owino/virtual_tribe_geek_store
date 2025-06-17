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
    let query = format!("SELECT * FROM volume WHERE comic = comic:{comic}"); // TODO: Order this list
    debug!("QUERY -> {query}");
    let mut volumes = db.query(query).await?;
    let mut volumes: Vec<Volume> = volumes.take(0)?;
    // volumes.sort_by(|a,b| 
    //     b.vol_no.cmp(&a.vol_no)
    // );
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
    // let id: &str = volume_for_create.comic.split(":").collect::<Vec<&str>>()[1];
    let id = "id";
    let query = format!("SELECT count() FROM volume WHERE comic = comic:{id}");
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
    //let count = count[1].count + 1;

    let vol = VolumeForCreateCount::from_vol_create(volume_for_create, count2);
    let volume: Option<DbId> = db.create("volume").content(vol).await?;
    eprintln!("Created {:#?}", volume);

    if let Some(v) =  volume{
        Ok(v)
    }else{
        Err(VolumeError::FailedToCreate)
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
