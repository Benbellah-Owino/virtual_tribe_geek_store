use std::io::Take;

use serde::{Deserialize, Serialize};
use surrealdb::sql::{value, Thing};
mod controllers;
pub mod routers;

// section:      -- structs
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Studio {
    pub name: String,
    pub owner: String,
    pub email: String,
    pub description: Option<String>, //More fields cam be addede
}

impl Studio{
    fn from(value: &Studio) -> Self {
        let val = value.description.to_owned();
        
        Studio{
            name: value.name.to_string(),
            owner: value.owner.to_string(),
            email: value.email.to_string(),
            description: val,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StudioForCreate {
    pub name: String,
    pub owner: String,
    pub email: String,
    //More fields cam be addede
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StudioForCreateForward {
    pub name: String,
    pub owner: Thing,
    pub email: String,
    //More fields cam be addede
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OwnerId{
    id: Thing
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StudioForUpdate {
    pub field: String,
    pub value: String
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StudioUpdateClient {
    pub payload: Vec<StudioForUpdate>
}
// endsection:   -- structs


// section:      -- enums
#[derive(Debug, Clone)]
pub enum StudioError {
    OwnerMismatch,
    //Create
    CreateStudioError,

    //Get
    StudioRetrievingError,
    StudioNotFoundError,
    NoStudiosError,

    //Update
    UpdateStudioError,

    //Delete
    DeleteStudioError,
}
// endsection:   -- enums


// section:      -- tests
#[cfg(test)]
mod tests {
use axum::routing::delete;
use serial_test::serial;
use surrealdb::sql::Thing;

use crate::dev_initial::db::connect_db;
use crate::studio::controllers::{create, update, get_all, get_details, delete_studio};

use super::*;

    #[serial]
    #[tokio::test]
    async fn studio_create() {
        
        let db = connect_db()
            .await
            .expect("TEST ERROR:-> Connection to db failed");
        //generate owner first
        let new_studio  = StudioForCreate{ 
            name: "test_studio".to_owned(),
            owner: "creator:3o18oyy2e8jug4r3p5d6".to_owned(), //Enter surrealdb creator id
            email: "test_studio@gmail.com".to_owned() 
        };

        let owner =  "creator:3o18oyy2e8jug4r3p5d6".to_owned(); //Enter surrealdb creator id
        let studio = create(&db, new_studio, owner).await.unwrap();
        let st = &studio[0];

        let _delete:Result<Vec<Studio>, surrealdb::Error> = db.delete("studio").await;
        assert_eq!("test_studio".to_string(), st.name );
        assert_eq!("test_studio@gmail.com".to_string(), st.email );
    }

    #[serial]
    #[tokio::test]
    async fn studio_get_all() {
        let db = connect_db()
            .await
            .expect("TEST ERROR:-> Connection to db failed");
        //generate owner first
        let new_studio  = StudioForCreate{ 
            name: "test_studio".to_owned(),
        owner: "creator:3o18oyy2e8jug4r3p5d6".to_owned(), //Enter surrealdb creator id
            email: "test_studio@gmail.com".to_owned() 
        };
        let new_studio2  = StudioForCreate{ 
            name: "test_studio2".to_owned(),
            owner: "creator:3o18oyy2e8jug4r3p5d6".to_owned(), //Enter surrealdb creator id
            email: "test_studio2@gmail.com".to_owned() 
        };

        let studio = create(&db, new_studio,"creator:3o18oyy2e8jug4r3p5d6".to_owned()).await.unwrap();
        
        let _delete:Result<Vec<Studio>, surrealdb::Error> = db.delete("studio").await;
        assert_eq!(2 as usize, studio.len());

    }

    #[serial]
    #[tokio::test]
    #[ignore]
    async fn studio_get_one() {
        let db = connect_db()
            .await
            .expect("TEST ERROR:-> Connection to db failed");
        //generate owner first
        let new_studio  = StudioForCreate{ 
            name: "test_studio".to_owned(),
            owner: "creator:3o18oyy2e8jug4r3p5d6".to_owned(), //Enter surrealdb creator id
            email: "test_studio@gmail.com".to_owned() 
        };

        let owner =  "creator:3o18oyy2e8jug4r3p5d6".to_owned();
        let studio = create(&db, new_studio, owner).await.unwrap();
        
        let _delete:Result<Vec<Studio>, surrealdb::Error> = db.delete("studio").await;

    }

    #[serial]
    #[tokio::test]
    #[ignore]
   async fn studio_get_all_creators() {

   }

    #[serial]
    #[tokio::test]
   async fn studio_update() {
        #[ignore]
        let db = connect_db()
            .await
            .expect("TEST ERROR:-> Connection to db failed");
        //generate owner first
        let new_studio  = StudioForCreate{ 
            name: "test_studio".to_owned(),
            owner: "creator:3o18oyy2e8jug4r3p5d6".to_owned(), //Enter surrealdb creator id
            email: "test_studio@gmail.com".to_owned() 
        };

       // let studio_for_update = StudioForUpdate{}
        let owner =  "creator:3o18oyy2e8jug4r3p5d6".to_owned();

        let studio = create(&db, new_studio, owner).await.unwrap();
    
        let st = &studio[0];

        let _delete:Result<Vec<Studio>, surrealdb::Error> = db.delete("studio").await;
        assert_eq!("test_studio".to_string(), st.name );
        assert_eq!("test_studio@gmail.com".to_string(), st.email );
}

    #[serial]
    #[tokio::test]
    async fn studio_delete() {}

    #[serial]
    #[tokio::test]
    async fn studio_details() {}
}
// endsection:   -- tests
