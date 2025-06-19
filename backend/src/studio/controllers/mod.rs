use serde_json::json;
// section:     -- imports
use surrealdb::{engine::remote::ws::Client, Surreal};
use tracing::debug;

use crate::helpers::db::thing_from_string;

use super::{
    Studio, StudioError, StudioForCreate, StudioForCreateForward, StudioForUpdate, StudioFull,
};

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
    studio: StudioForCreate,
) -> Result<StudioFull, StudioError> {
    let owner = studio.owner.clone();

    println!("{owner}");
    //Extracting the id string

    let creator = thing_from_string(owner);
    println!("CREATOR: {creator}");

    let st: StudioForCreateForward = StudioForCreateForward {
        name: studio.name.clone(),
        owner: creator,
        email: studio.email.clone(),
        description: studio.description,
    };

    let new_studio: Result<Option<StudioFull>, surrealdb::Error> =
        db.create("studio").content(st).await;
    println!("{:?}", new_studio);
    match new_studio {
        Ok(s) => {
            if let Some(g) = s {
                Ok(g)
            } else {
                Err(StudioError::StudioRetrievingError)
            }
        }
        Err(e) => {
            dbg!(&e);
            Err(StudioError::CreateStudioError)
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
pub async fn get_all(db: &Surreal<Client>) -> Result<Vec<Studio>, StudioError> {
    let studios: Result<Vec<Studio>, surrealdb::Error> = db.select("studio").await;
    println!("TAG 1");
    println!("{:?}", studios);
    match studios {
        Ok(s) => {
            debug!("{:?}", s);
            println!("TAG2");
            Ok(s)
        }
        Err(e) => {
            debug!("{:?}", e);
            Err(StudioError::StudioRetrievingError)
        }
    }
}

/// Controller for getting details of one studio
///
/// # Example
/// fn get_studio{
///
/// }
pub async fn get_details(db: &Surreal<Client>, id: String) -> Result<Studio, StudioError> {
    println!(
        "\n\n{:<12}=====================================================================\n\n",
        "studio::get_details()"
    );
    let id: &str = id.split(":").collect::<Vec<&str>>()[1];
    debug!("{id}");
    let studios: Result<Option<Studio>, surrealdb::Error> = db.select(("studio", id)).await;

    println!("{:?}", studios);
    match studios {
        Ok(s) => {
            if let Some(s) = s {
                //An extra let to ensure studio is returned
                debug!("{:?}", s);
                Ok(s)
            } else {
                Err(StudioError::StudioRetrievingError)
            }
        }
        Err(e) => {
            debug!("{:?}", e);
            Err(StudioError::StudioRetrievingError)
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
#[allow(dead_code)]
pub async fn get_all_creators(_db: &Surreal<Client>) {
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
pub async fn update(
    db: &Surreal<Client>,
    id: &String,
    studio: StudioForUpdate,
) -> Result<Studio, StudioError> {
    println!(
        "\n\n{:<12}=====================================================================\n\n",
        "studio::update()"
    );
    let id: &str = id.split(":").collect::<Vec<&str>>()[1];
    let updated_studio: Result<Option<Studio>, surrealdb::Error> = db
        .update(("studio", id))
        .merge(json!({&studio.field: &studio.value}))
        .await;

    match updated_studio {
        Ok(s) => {
            if let Some(s) = s {
                //An extra let to ensure studio is returned
                debug!("{:?}", s);
                Ok(s)
            } else {
                Err(StudioError::StudioRetrievingError)
            }
        }
        Err(e) => {
            debug!("{:?}", e);
            Err(StudioError::StudioRetrievingError)
        }
    }
}

/// Controller for deleting studio
///
/// # Example
/// fn delete_studio{
///
/// }
pub async fn delete_studio(db: &Surreal<Client>, id: String) -> Result<Studio, StudioError> {
    println!(
        "\n\n{:<12}=====================================================================\n\n",
        "studio::delete_studio()"
    );
    let id: &str = id.split(":").collect::<Vec<&str>>()[1];
    let deleted_studio: Result<Option<Studio>, surrealdb::Error> = db.delete(("studio", id)).await;

    match deleted_studio {
        Ok(s) => {
            if let Some(s) = s {
                //An extra let to ensure studio is returne
                debug!("{:?}", s);
                Ok(s)
            } else {
                Err(StudioError::StudioRetrievingError)
            }
        }
        Err(e) => {
            debug!("{:?}", e);
            Err(StudioError::StudioRetrievingError)
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
