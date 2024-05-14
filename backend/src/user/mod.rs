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
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
    pub refresh_token: Option<String>,
    pub login_attempts: u8,
    pub joined_at: String,
    pub verified: bool,
    pub avatar: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserForCreate {
    pub username: String,
    pub email: String,
    pub password: String,
}
impl UserForCreate {
    pub fn new(username: &str, email: &str, role: Vec<&str>, password: &str) -> UserForCreate {
        let mut roles: Vec<String> = vec![];
        for i in role {
            roles.push(String::from(i));
        }
        return UserForCreate {
            username: String::from(username),
            email: String::from(email),
            password: String::from(password),
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserForLogin {
    pub email: String,
    pub password: String,
}
// login struct
#[derive(Debug, Serialize, Deserialize)]
pub struct UserForLoginSuccess {
    pub id: Thing,
    pub email: String,
    pub username: String,
    pub password: String,
    pub verified: bool,
    pub login_attempts: u8,
}

// Items to update username, password, 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserForUpdateClient {
    pub field: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserForUpdateDb {
    pub email: String,
    pub username: String,
}

// endection:   -- structs

// section:     -- error
#[derive(Debug, Serialize, Deserialize)]
pub enum UserError {
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

// #[cfg(test)]
// mod tests {
//     use crate::dev_initial::db::connect_db;

//     use self::controllers::{login, register};

//     use super::*;

//     #[tokio::test]
//     async fn User_registration() {
//         let db = connect_db()
//             .await
//             .expect("TEST ERROR:-> Connection to db failed");
//         let creater = UserForCreate {
//             username: String::from("TestUser"),
//             email: String::from("testUser@gmail.com"),
//             role: vec![String::from("writer")],
//             password: String::from("password"),
//         };
//         let t = register(&db, creater).await.unwrap();
//         let t = &t[0];
//         let _delete: Result<Vec<UserForCreate>, surrealdb::Error> = db.delete("User").await;

//         assert_eq!(String::from("TestUser"), t.username);
//         assert_eq!(String::from("testUser@gmail.com"), t.email);
//         assert_eq!(vec![String::from("writer")], t.role);
//         assert_eq!(String::from("password"), t.password);
//     }

//     #[tokio::test]
//     async fn User_login() {
//         let db = connect_db()
//             .await
//             .expect("TEST ERROR:-> Connection to db failed");

//         let creater = UserForCreate {
//             email: String::from("testUser@gmail.com"),
//             username: String::from("TestUser"),
//             role: vec![String::from("writer")],
//             password: String::from("password"),
//         };
//         let _new_User = register(&db, creater).await.unwrap();

//         let User = UserForLogin {
//             email: String::from("testUser@gmail.com"),
//             password: String::from("password"),
//         };

//         let logged_in_User = login(&db, User).await.unwrap();

//         let _delete: Result<Vec<UserForCreate>, surrealdb::Error> = db.delete("User").await;

//         assert_eq!(String::from("TestUser"), logged_in_User.username);
//         assert_eq!(
//             String::from("testUser@gmail.com"),
//             logged_in_User.email
//         );
//         assert_eq!(String::from("User"), logged_in_User.acc_type);
//     }
//}
// // endsection:  -- tests

