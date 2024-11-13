use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

mod controllers;
pub mod routers;

// region:      -- structs
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Studio {
    pub name: String,
    pub owner: Thing,
    pub email: String,
    pub description: Option<String>, //More fields cam be addede
}

impl Studio {
    fn from(value: &Studio) -> Self {
        let val = value.description.to_owned();

        Studio {
            name: value.name.to_string(),
            owner: value.owner.to_owned(),
            email: value.email.to_string(),
            description: val,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StudioFull {
    pub id: Thing,
    pub name: String,
    pub owner: Thing,
    pub email: String,
    pub description: Option<String>, //More fields cam be addede
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
pub struct OwnerId {
    id: Thing,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StudioForUpdate {
    pub field: String,
    pub value: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StudioUpdateClient {
    pub payload: Vec<StudioForUpdate>,
}
// endregion:   -- structs

// region:      -- enums
#[derive(Debug, Clone)]
pub enum StudioError {
    OwnerMismatch,
    /*
    Create
    */
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
// endregion:   -- enums

// region:      -- tests
#[cfg(test)]
mod tests {
    use serial_test::serial;
    use surrealdb::engine::remote::ws::Client;
    use surrealdb::Surreal;

    use crate::creator::CreatorForCreate;
    use crate::dev_initial::db::connect_db;
    use crate::studio::controllers::create;
    use crate::vrt_lib::ItemId;

    use super::*;

    pub struct TestErr;
    async fn creator_setup(db: &Surreal<Client>) -> ItemId {
        let creator_for_create = CreatorForCreate {
            username: "TestUser".to_string(),
            email: "UTestUser@test.com".to_string(),
            role: "Writer".to_string(),
            password: "Password".to_string(),
        };

        let created: Result<Vec<ItemId>, surrealdb::Error> =
            db.create("creator").content(creator_for_create).await;

        if let Ok(c) = created {
            return c[0].clone();
        } else {
            panic!("Failed to create creator");
        }
    }
    #[serial]
    #[tokio::test]
    async fn studio_create() {
        let db = connect_db()
            .await
            .expect("TEST ERROR:-> Connection to db failed");
        //generate owner first
        let creator = creator_setup(&db).await;
        let new_studio = StudioForCreate {
            name: "test_studio".to_owned(),
            owner: creator.id.to_string(), //Enter surrealdb creator id
            email: "test_studio@gmail.com".to_owned(),
        };

        let studio = create(&db, new_studio).await.unwrap();
        let st = &studio[0];

        let _delete: Result<Vec<Studio>, surrealdb::Error> = db.delete("studio").await;
        assert_eq!("test_studio".to_string(), st.name);
        assert_eq!("test_studio@gmail.com".to_string(), st.email);
    }

    #[serial]
    #[tokio::test]
    async fn studio_get_all() {
        let db = connect_db()
            .await
            .expect("TEST ERROR:-> Connection to db failed");
        //generate owner first
        let creator = creator_setup(&db).await;
        let new_studio = StudioForCreate {
            name: "test_studio".to_owned(),
            owner: creator.id.to_string(), //Enter surrealdb creator id
            email: "test_studio@gmail.com".to_owned(),
        };
        let new_studio2 = StudioForCreate {
            name: "test_studio2".to_owned(),
            owner: creator.id.to_string(), //Enter surrealdb creator id
            email: "test_studio2@gmail.com".to_owned(),
        };

        let studio = create(&db, new_studio).await.unwrap();
        let studio2 = create(&db, new_studio2).await.unwrap();

        let _delete: Result<Vec<Studio>, surrealdb::Error> = db.delete("studio").await;
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
        let creator = creator_setup(&db).await;
        let new_studio = StudioForCreate {
            name: "test_studio".to_owned(),
            owner: creator.id.to_string(), //Enter surrealdb creator id
            email: "test_studio@gmail.com".to_owned(),
        };
        let studio = create(&db, new_studio.clone()).await.unwrap();

        let _delete: Result<Vec<Studio>, surrealdb::Error> = db.delete("studio").await;
        assert_eq!(studio[0].name, new_studio.name);
    }

    // #[serial]
    // #[tokio::test]
    // #[ignore]
    // async fn studio_get_all_creators() {}

    // #[serial]
    // #[tokio::test]
    // async fn studio_update() {
    //     #[ignore]
    //     let db = connect_db()
    //         .await
    //         .expect("TEST ERROR:-> Connection to db failed");
    //     //generate owner first

    //     let creator = creator_setup(&db).await;
    //     let new_studio = StudioForCreate {
    //         name: "test_studio".to_owned(),
    //         owner: creator.id.to_string(), //Enter surrealdb creator id
    //         email: "test_studio@gmail.com".to_owned(),
    //     };
    //     let studio = create(&db, new_studio.clone()).await.unwrap();
    //     // let studio_for_update = StudioForUpdate{}

    //     let st = &studio[0];

    //     let _delete: Result<Vec<Studio>, surrealdb::Error> = db.delete("studio").await;
    //     assert_eq!("test_studio".to_string(), st.name);
    //     assert_eq!("test_studio@gmail.com".to_string(), st.email);
    // }

    // #[serial]
    // #[tokio::test]
    // async fn studio_delete() {}

    // #[serial]
    // #[tokio::test]
    // async fn studio_details() {}
}
// endregion:   -- tests
