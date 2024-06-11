use serde_json::json;
// section:     -- imports
use surrealdb::{engine::remote::ws::Client, opt::RecordId, sql::{Id, Thing}, Surreal};
use tracing::debug;

use crate::creator;

use super::{OwnerId, Studio, StudioError, StudioForCreate, StudioForCreateForward, StudioForUpdate};

/// Controller for creating studio
///
/// # Example
/// ```
/// fn create_studio{
///       let new_studio: Result<Vec<StudioForCreate>, surrealdb::Error> = db.create("studio").content(studio).await;
/// }
/// ```
pub async fn create(
    db: &Surreal<Client>,
    mut studio: StudioForCreate,
    creator: String
) -> Result<Vec<StudioForCreateForward>, StudioError> {
    
    let owner = studio.owner.clone();
    
    //Extracting the id string
     
    let id_string:Vec<&str> = owner.split(':').collect();
    let rec_id_string = id_string[1].to_string();
    let creator = Thing{
        tb : "creator".to_string(),
        id: Id::from(id_string[1].to_string())
    };
    
    //TODO: Fix this;
    let creator:Option<OwnerId> = db.select(creator).await.unwrap();
    // if let Some(c) = creator{
    //     // studio.owner = OwnerId::Thing(c);
        
    //     c
    // };

    let ct = creator.unwrap();

    let st: StudioForCreateForward = StudioForCreateForward{
        name : studio.name.clone(),
        owner: ct.id,
        email: studio.email.clone(),
    };

    let new_studio: Result<Vec<StudioForCreateForward>, surrealdb::Error> =
        db.create("studio").content(st).await;
    println!("{:?}", new_studio);
    match new_studio {
        Ok(s) => {
            dbg!(&s);
            return Ok(s);
        }
        Err(e) => {
            dbg!(&e);
            return Err(StudioError::CreateStudioError);
        }
    }
}

/// Controller for geting all studios
///
/// # Example
/// ``````
/// fn get_all_studio{
///    let new_studio: Result<Vec<StudioForCreate>, surrealdb::Error> = db.create("studio").content(studio).await;
/// }
/// ```
pub async fn get_all(db: &Surreal<Client>) -> Result<Vec<StudioForCreate>, StudioError> {
    let studios: Result<Vec<StudioForCreate>, surrealdb::Error> = db.select("studio").await;

    match studios {
        Ok(s) => {
            debug!("{:?}", s);
            return Ok(s);
        }
        Err(e) => {
            debug!("{:?}", e);
            return Err(StudioError::StudioRetrievingError);
        }
    }
}

/// Controller for getting details of one studio
///
/// # Example
/// fn get_studio{
///
/// }
pub async fn get_details(db: &Surreal<Client>, id: String) -> Result<StudioForCreate, StudioError> {
    println!(
        "\n\n{:<12}=====================================================================\n\n",
        "studio::get_details()"
    );
    let id: &str = id.split(":").collect::<Vec<&str>>()[1];
    let studios: Result<Option<StudioForCreate>, surrealdb::Error> =
        db.select(("studio", id)).await;

    match studios {
        Ok(s) => {
            if let Some(s) = s {//An extra let to ensure studio is returned
                debug!("{:?}", s);
                return Ok(s);
            } else {
                return Err(StudioError::StudioRetrievingError);
            }
        }
        Err(e) => {
            debug!("{:?}", e);
            return Err(StudioError::StudioRetrievingError);
        }
    }
}

/// Controller for getting all creators working at a studio
///
/// # Example
/// ```
/// fn get_studio_creators{
///
/// }
/// ```
pub async fn get_all_creators(db: &Surreal<Client>) {
    todo!()
}

/// Controller for updating studio
///
/// # Example
/// ```
/// fn update_studio{
///     let updated = update(db, id, studio_for_update);
/// }
/// ```
pub async fn update(db: &Surreal<Client>,id:&String, studio: StudioForUpdate) -> Result<Studio, StudioError> {
    println!(
        "\n\n{:<12}=====================================================================\n\n",
        "studio::update()"
    );
    let id: &str = id.split(":").collect::<Vec<&str>>()[1];
    let updated_studio:Result<Option<Studio>, surrealdb::Error> = db.update(("studio", id)).merge(json!({&studio.field: &studio.value})).await ;
    
    
    match updated_studio {
        Ok(s) => {
            if let Some(s) = s { //An extra let to ensure studio is returned
                debug!("{:?}", s);
                return Ok(s);
            } else {
                return Err(StudioError::StudioRetrievingError);
            }
        }
        Err(e) => {
            debug!("{:?}", e);
            return Err(StudioError::StudioRetrievingError);
        }
    }
}

/// Controller for deleting studio
///
/// # Example
/// fn delete_studio{
///
/// }
pub async fn delete_studio(db: &Surreal<Client>, id:String) -> Result<Studio, StudioError> {
    println!(
        "\n\n{:<12}=====================================================================\n\n",
        "studio::delete_studio()"
    );
    let id: &str = id.split(":").collect::<Vec<&str>>()[1];
    let deleted_studio:Result<Option<Studio>, surrealdb::Error> = db.delete(("studio", id)).await;

    match deleted_studio {
        Ok(s) => {
            if let Some(s) = s { //An extra let to ensure studio is returne
                debug!("{:?}", s);
                return Ok(s);
            } else {
                return Err(StudioError::StudioRetrievingError);
            }
        }
        Err(e) => {
            debug!("{:?}", e);
            return Err(StudioError::StudioRetrievingError);
        }
    }
    
}

/// Controller for creating studio
///
/// # Example
/// fn create_studio{
///
/// }
///
pub fn studio() {
    println!("Get student")
}
