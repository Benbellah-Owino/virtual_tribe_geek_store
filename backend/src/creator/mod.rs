// section:     -- imports
use serde::{Deserialize, Serialize};
// endection:   -- imports

// section:     -- mods
mod controllers;
pub mod routers;
// endection:   -- mods


// section:      -- types
// endsection:   -- types

// section:     -- structs
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreatorForCreate{
    pub username: String,
    pub email:String,
    pub role: Vec<String>,
    pub password: String
}

// login struct
#[derive(Debug, Serialize, Deserialize)]
pub struct CreatorForLogin{
    pub username: String,
    pub email:String,
    pub role: Vec<String>,
    pub describe: String,
    pub verified: String
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
    LoginError,

    //
    
}
// endsection:   -- error



// section:     -- imports
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
    
}
// endsection:  -- imports