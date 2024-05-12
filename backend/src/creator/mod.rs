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
pub struct Socials{
    pub facebook: String,
    pub twitter_x: String,
    pub instagram: String
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Creator{
    pub username: String,
    pub email: String,
    pub role: Vec<String>,
    pub password: String,
    pub description: Option<String>,
    pub verified: bool,
    pub refresh_token: Option<String>,
    pub socials: Option<Socials>,
    pub login_attempts: i8,
    pub joined_at : String,
    pub avatar: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreatorForCreate{
    pub username: String,
    pub email: String,
    pub role: Vec<String>,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatorForLogin{
    pub email: String,
    pub password: String
}
// login struct
#[derive(Debug, Serialize, Deserialize)]
pub struct CreatorForLoginSuccess{
    pub id: Thing,
    pub email: String,
    pub username:String,
    pub password: String,
    pub verified: bool,
    pub login_attempts: u8
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreatorForUpdate<I: Serialize + for<'a> Deserialize<'a>>{
    pub field: String,
    pub value: I
}

impl CreatorForCreate{
    pub fn new(username: &str, email: &str, role:Vec<&str>, password: &str) -> CreatorForCreate {
        let mut roles:Vec<String> = vec![];
        for i in role{
            roles.push(String::from(i));
        }
        return CreatorForCreate{ 
            username: String::from(username),
            email: String::from(email),
            role: roles,
            password: String::from(password), 
        }
    }
}
// endection:   -- structs



// section:     -- error
#[derive(Debug, Serialize, Deserialize)]
pub enum CreatorError{
    RegistrationError,
    
    //Login Errors
    LoginError,
    WrongCredentialsError,
    LoginAttemptsError,
    
    //Details Errors
    DetailsRetrievingError,
    DetailsUpdateError
    
}
// endsection:   -- error



// section:     -- tests

#[cfg(test)]
mod tests {
    use crate::dev_initial::db::connect_db;

    use self::controllers::{register, login};

    use super::*;

    #[tokio::test]
    async fn creator_registration(){
        let db = connect_db().await.expect("TEST ERROR:-> Connection to db failed");
        let creater = CreatorForCreate{ 
            username: String::from("TestCreator"),
            email: String::from("testcreator@gmail.com"),
            role: vec![String::from("writer")],
            password: String::from("password"), 
    };
        let t = register(&db, creater).await.unwrap();
        let t = &t[0];
        let _delete:Result<Vec<CreatorForCreate>, surrealdb::Error> = db.delete("creator").await;

    assert_eq!(String::from("TestCreator"), t.username);    
    assert_eq!(String::from("testcreator@gmail.com"), t.email);    
    assert_eq!(vec![String::from("writer")], t.role);    
    assert_eq!(String::from("password"), t.password);    
    }


    #[tokio::test]
    async fn creator_login(){
        let db = connect_db().await.expect("TEST ERROR:-> Connection to db failed");
        
        let creater = CreatorForCreate{ 
            email: String::from("testcreator@gmail.com"),
            username: String::from("TestCreator"),
            role: vec![String::from("writer")],
            password: String::from("password"), 
        };
        let _new_creator = register(&db, creater).await.unwrap();

        let creator =  CreatorForLogin{ 
            
            email: String::from("testcreator@gmail.com"),
            password: String::from("password")
        };

        let logged_in_creator = login(&db, creator).await.unwrap();

        let _delete:Result<Vec<CreatorForCreate>, surrealdb::Error> = db.delete("creator").await;
        
        assert_eq!(String::from("TestCreator"), logged_in_creator.username);    
        assert_eq!(String::from("testcreator@gmail.com"), logged_in_creator.email);    
        assert_eq!(String::from("creator"), logged_in_creator.acc_type);    
    }

    
}
// endsection:  -- tests