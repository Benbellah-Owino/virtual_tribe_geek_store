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
#[cfg(test)]
mod tests {
    use serial_test::serial;

    use crate::dev_initial::db::connect_db;

    use self::controllers::{delete_user, get_details, login, register, update_details};

    use super::*;

    #[serial]
    #[tokio::test]
    async fn user_registration() {
        let db = connect_db()
            .await
            .expect("TEST ERROR:-> Connection to db failed");
        let creater = UserForCreate {
            username: String::from("Testuser"),
            email: String::from("testuser@gmail.com"),
            password: String::from("password"),
        };
        let t = register(&db, creater).await.unwrap();
        let _delete: Result<Vec<UserForCreate>, surrealdb::Error> = db.delete("user").await;

        assert_eq!(String::from("Testuser"), t.username);
        assert_eq!(String::from("testuser@gmail.com"), t.email);
        assert_eq!(String::from("password"), t.password);
    }

    #[serial]
    #[tokio::test]
    async fn user_login() {
        let db = connect_db()
            .await
            .expect("TEST ERROR:-> Connection to db failed");

        let creater = UserForCreate {
            email: String::from("testuser@gmail.com"),
            username: String::from("Testuser"),
            password: String::from("password"),
        };
        let _new_user = register(&db, creater).await.unwrap();

        let user = UserForLogin {
            email: String::from("testuser@gmail.com"),
            password: String::from("password"),
        };

        let logged_in_user = login(&db, user).await.unwrap();

        let _delete: Result<Vec<UserForCreate>, surrealdb::Error> = db.delete("user").await;

        assert_eq!(String::from("Testuser"), logged_in_user.username);
        assert_eq!(String::from("testuser@gmail.com"), logged_in_user.email);
        assert_eq!(String::from("user"), logged_in_user.acc_type);
    }

    #[serial]
    #[tokio::test]
    async fn user_update() {
        let db = connect_db()
            .await
            .expect("TEST ERROR:-> Connection to db failed");
        //Create user
        let creater = UserForCreate {
            email: String::from("testuser@gmail.com"),
            username: String::from("Testuser"),
            password: String::from("password"),
        };
        let _new_user = register(&db, creater).await.unwrap();

        //loggin user to get id
        let user_for_login = UserForLogin {
            email: String::from("testuser@gmail.com"),
            password: String::from("password"),
        };

        let logged_user = login(&db, user_for_login).await.unwrap();

        let user_for_update = UserForUpdateClient {
            field: String::from("email"),
            value: String::from("testuserupdated@gmail.com"),
        };

        let updated_user = update_details(&db, logged_user.id, user_for_update)
            .await
            .unwrap();

        let _delete: Result<Vec<UserForCreate>, surrealdb::Error> = db.delete("user").await;

        dbg!(&updated_user.email);
        assert_eq!(
            String::from("testuserupdated@gmail.com"),
            updated_user.email
        );
    }

    #[serial]
    #[tokio::test]
    async fn user_details() {
        let db = connect_db()
            .await
            .expect("TEST ERROR:-> Connection to db failed");
        //Create user
        let creater = UserForCreate {
            email: String::from("testuser@gmail.com"),
            username: String::from("Testuser"),
            password: String::from("password"),
        };
        let _new_user = register(&db, creater).await.unwrap();

        //loggin user to get id
        let user_for_login = UserForLogin {
            email: String::from("testuser@gmail.com"),
            password: String::from("password"),
        };

        let logged_user = login(&db, user_for_login).await.unwrap();

        let user = get_details(&db, logged_user.id).await.unwrap();

        let _delete: Result<Vec<UserForCreate>, surrealdb::Error> = db.delete("user").await;

        assert_eq!(String::from("testuser@gmail.com"), user.email);
        assert_eq!(String::from("Testuser"), user.username);
    }

    #[serial]
    #[tokio::test]
    async fn user_delete() {
        let db = connect_db()
            .await
            .expect("TEST ERROR:-> Connection to db failed");
        //Create user
        let creater = UserForCreate {
            email: String::from("testuser@gmail.com"),
            username: String::from("Testuser"),
            password: String::from("password"),
        };
        let _new_user = register(&db, creater).await.unwrap();

        //loggin user to get id
        let user_for_login = UserForLogin {
            email: String::from("testuser@gmail.com"),
            password: String::from("password"),
        };

        let logged_user = login(&db, user_for_login).await.unwrap();

        let delete = delete_user(&db, logged_user.id).await.unwrap();

        assert_eq!(String::from("testuser@gmail.com"), delete.email);
        assert_eq!(String::from("Testuser"), delete.username);
    }
}
