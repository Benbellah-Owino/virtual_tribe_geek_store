// section:     -- imports
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
// endection:   -- imports

// section:     -- mods
mod controllers;
pub mod routers;
// endection:   -- mods

// section:      -- types
// endsection:   -- types

// section:     -- structs
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Socials {
    pub facebook: Option<String>,
    pub twitter_x: Option<String>,
    pub instagram: Option<String>,
}

impl Socials {
    fn new() -> Socials {
        Socials {
            facebook: None,
            twitter_x: None,
            instagram: None,
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Creator {
    pub username: String,
    pub email: String,
    pub role: String,
    pub password: String,
    pub description: Option<String>,
    pub verified: bool,
    pub refresh_token: Option<String>,
    pub socials: Option<Socials>,
    pub login_attempts: i8,
    pub joined_at: String,
    pub avatar: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreatorDetails {
    pub username: String,
    pub email: String,
    pub role: String,
    pub description: Option<String>,
    pub verified: bool,
    pub socials: Option<Socials>,
    pub joined_at: String,
    pub avatar: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreatorForCreate {
    pub username: String,
    pub email: String,
    pub role: String,
    pub password: String,
}
impl CreatorForCreate {
    pub fn new(username: &str, email: &str, role: &str, password: &str) -> CreatorForCreate {
        return CreatorForCreate {
            username: String::from(username),
            email: String::from(email),
            role: String::from(role),
            password: String::from(password),
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatorForLogin {
    pub email: String,
    pub password: String,
}
// login struct
#[derive(Debug, Serialize, Deserialize)]
pub struct CreatorForLoginSuccess {
    pub id: Thing,
    pub email: String,
    pub username: String,
    pub password: String,
    pub verified: bool,
    pub login_attempts: u8,
}

// Items to update username, password, socials, description,
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatorForUpdateClient {
    pub field: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatorForUpdateDb {
    pub email: String,
    pub username: String,
    pub description: Option<String>,
    pub socials: Option<Socials>,
}

// endection:   -- structs

// section:     -- error
#[derive(Debug, Serialize, Deserialize)]
pub enum CreatorError {
    RegistrationError,

    //Login Errors
    LoginError,
    WrongCredentialsError,
    LoginAttemptsError,

    //Details Errors
    DetailsRetrievingError,
    DetailsUpdateError,

    //Updating Error
    UpdatingError,
    WrongFieldError,

    //Deleting Error
    DeletingError,
}
// endsection:   -- error

// section:     -- tests

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use crate::dev_initial::db::connect_db;

    use self::controllers::{delete_creator, get_details, login, register, update_details};

    use super::*;

    #[serial]
    #[tokio::test]
    async fn creator_registration() {
        let db = connect_db()
            .await
            .expect("TEST ERROR:-> Connection to db failed");
        let creater = CreatorForCreate {
            username: String::from("TestCreator"),
            email: String::from("testcreator@gmail.com"),
            role: String::from("writer"),
            password: String::from("password"),
        };
        let t = register(&db, creater).await.unwrap();
        let t = &t[0];
        let _delete: Result<Vec<CreatorForCreate>, surrealdb::Error> = db.delete("creator").await;

        assert_eq!(String::from("TestCreator"), t.username);
        assert_eq!(String::from("testcreator@gmail.com"), t.email);
        assert_eq!(String::from("writer"), t.role);
        assert_eq!(String::from("password"), t.password);
    }

    #[serial]
    #[tokio::test]
    async fn creator_login() {
        let db = connect_db()
            .await
            .expect("TEST ERROR:-> Connection to db failed");

        let creater = CreatorForCreate {
            email: String::from("testcreator@gmail.com"),
            username: String::from("TestCreator"),
            role: String::from("writer"),
            password: String::from("password"),
        };
        let _new_creator = register(&db, creater).await.unwrap();

        let creator = CreatorForLogin {
            email: String::from("testcreator@gmail.com"),
            password: String::from("password"),
        };

        let logged_in_creator = login(&db, creator).await.unwrap();

        let _delete: Result<Vec<CreatorForCreate>, surrealdb::Error> = db.delete("creator").await;

        assert_eq!(String::from("TestCreator"), logged_in_creator.username);
        assert_eq!(
            String::from("testcreator@gmail.com"),
            logged_in_creator.email
        );
        assert_eq!(String::from("creator"), logged_in_creator.acc_type);
    }

    #[serial]
    #[tokio::test]
    async fn creator_update() {
        let db = connect_db()
            .await
            .expect("TEST ERROR:-> Connection to db failed");
        //Create creator
        let creater = CreatorForCreate {
            email: String::from("testcreator@gmail.com"),
            username: String::from("TestCreator"),
            role: String::from("writer"),
            password: String::from("password"),
        };
        let _new_creator = register(&db, creater).await.unwrap();

        //loggin creator to get id
        let creator_for_login = CreatorForLogin {
            email: String::from("testcreator@gmail.com"),
            password: String::from("password"),
        };

        let logged_creator = login(&db, creator_for_login).await.unwrap();
        eprintln!("{:?}", logged_creator);
        let creator_for_update = CreatorForUpdateClient {
            field: String::from("email"),
            value: String::from("testcreatorupdated@gmail.com"),
        };

        match update_details(&db, logged_creator.id, creator_for_update).await {
            Ok(updated_creator) => {
                println!("{:?}", updated_creator);
                assert_eq!(
                    String::from("testcreatorupdated@gmail.com"),
                    updated_creator.email
                );
            }
            Err(e) => {
                eprintln!("{:?}", e);
                panic!("Update Error");
            }
        }

        let _delete: Result<Vec<CreatorForCreate>, surrealdb::Error> = db.delete("creator").await;
    }

    #[serial]
    #[tokio::test]
    async fn creator_details() {
        let db = connect_db()
            .await
            .expect("TEST ERROR:-> Connection to db failed");
        //Create creator
        let creater = CreatorForCreate {
            email: String::from("testcreator@gmail.com"),
            username: String::from("TestCreator"),
            role: String::from("writer"),
            password: String::from("password"),
        };
        let _new_creator = register(&db, creater).await.unwrap();

        //loggin creator to get id
        let creator_for_login = CreatorForLogin {
            email: String::from("testcreator@gmail.com"),
            password: String::from("password"),
        };

        let logged_creator = login(&db, creator_for_login).await;
        if let Ok(c) = logged_creator {
            println!("{:?}", c);
            let creator = get_details(&db, c.id).await.unwrap();

            let _delete: Result<Vec<CreatorForCreate>, surrealdb::Error> =
                db.delete("creator").await;

            assert_eq!(String::from("testcreator@gmail.com"), creator.email);
            assert_eq!(String::from("TestCreator"), creator.username);
        };
    }

    #[serial]
    #[tokio::test]
    async fn creator_delete() {
        let db = connect_db()
            .await
            .expect("TEST ERROR:-> Connection to db failed");
        //Create creator
        let creater = CreatorForCreate {
            email: String::from("testcreator@gmail.com"),
            username: String::from("TestCreator"),
            role: String::from("writer"),
            password: String::from("password"),
        };
        let _new_creator = register(&db, creater).await.unwrap();

        //loggin creator to get id
        let creator_for_login = CreatorForLogin {
            email: String::from("testcreator@gmail.com"),
            password: String::from("password"),
        };

        let logged_creator = login(&db, creator_for_login).await.unwrap();

        let delete = delete_creator(&db, logged_creator.id).await.unwrap();

        assert_eq!(String::from("testcreator@gmail.com"), delete.email);
        assert_eq!(String::from("TestCreator"), delete.username);
    }
}
// endsection:  -- tests
