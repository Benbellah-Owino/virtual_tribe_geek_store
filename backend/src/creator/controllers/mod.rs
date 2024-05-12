use serde_json::json;
// section:     -- imports
use surrealdb::{
    engine::remote::ws::Client, Surreal
};

use crate::{creator::{Creator, CreatorError, CreatorForLoginSuccess}, middleware::auth::jwt::Claims};

use super::{ CreatorForCreate, CreatorForLogin};
use tracing::{info, debug, error, warn};
use chrono::Utc;
use serde::{Deserialize, Serialize};
// endsection:   -- imports


// section:     -- mods
// endsection:   -- mods



// section:     -- controllers

/// Adds a new creator to the database
/// 
/// # Examples
/// 
/// ```
/// #[ignore]
/// async fn register(db:&Surreal<Client>){
/// let creator = CreatorForCreate{ 
///        username: String::from("TestCreator"),
///        email: String::from("testcreator@gmail.com"),
///        role: vec![String::from("writer")],
///        password: String::from("password"), 
///  };
/// let new_creator = creator::controllers::register(&db, creator);
/// let t = register(&db, creater).await.unwrap();
/// let t = &t[0];
/// let _delete:Result<Vec<CreatorForCreate>, surrealdb::Error> = db.delete("creator").await;
///
/// assert_eq!(String::from("TestCreator"), t.username);    
/// assert_eq!(String::from("testcreator@gmail.com"), t.email);    
/// assert_eq!(vec![String::from("writer")], t.role);    
/// assert_eq!(String::from("password"), t.password);
///}
/// ```
pub async fn register<'a>(db: &Surreal<Client>,creator:CreatorForCreate) -> Result<Vec<CreatorForCreate>, CreatorError>{

    println!("\n\n{:<12} ====================================================================\n\n", "creator::register()");
    let created: Result<Vec<CreatorForCreate>,surrealdb::Error>= db.create("creator")
                                        .content(creator)
                                        .await;

    match created{
        Ok(c) =>{
            debug!("{:<12} - registered new creator", "FOR-DEV-ONLY");
            println!("{:?}", c);

            println!("\n\n============================================================================================\n\n");
            return Ok(c)
        },
        Err(e)=>{
            error!("        - Creator registration error");
            dbg!(e);
            println!("\n\n============================================================================================\n\n\n");
            return Err(CreatorError::RegistrationError)
        }
    } 
    

}




/// Logs in creator and returns a Claims struct for tokenization
/// 
/// # Examples
/// 
/// ```
/// #[ignore]
///async fn login(db:&Surreal<Client>,creator: _){
/// let creator = CreatorForLogin{
///        email: String::from("testcreator@gmail.com"),
///        password: String::from("password"), 
/// };
/// 
/// let logged = creator::controllers::login(&db, creator);
/// let claim = login(&db, creator).await.unwrap();
/// 
/// let _delete:Result<Vec<Creator>, surrealdb::Error> = db.delete("creator").await;
///
/// assert_eq!(String::from("TestCreator"), t.username);    
/// assert_eq!(String::from("testcreator@gmail.com"), t.email);    
/// assert_eq!(vec![String::from("creator")], t.acc_type);    
/// assert_eq!(false, t.verified);
/// }
///
/// ```
pub async fn login(db: &Surreal<Client>,creator:CreatorForLogin) -> Result<Claims, CreatorError> {
    //lequery
    println!("\n\n{:<12}=====================================================================\n\n", "creator::login()");
    dbg!(&creator);
    let ct = db.
                                    query("SELECT id,email, role, password, username, verified, login_attempts FROM creator WHERE email = $email")
                                    .bind(("email",&creator.email))
                                    .await; 


    match ct{ //We format the result for comparison and send back appropriate result 
        Ok(mut c) =>{
            debug!("{:<12} - creator login", "FOR-DEV-ONLY");
            
            #[allow(unused_mut)]
            let mut res: Result<Vec<CreatorForLoginSuccess>, surrealdb::Error> = c.take(0); // Convert the result to a vector containing the creator for login
            dbg!(&res);
            
            match res{ //INNER MATCH ------------------------------------------------------------------
                Ok(t) => {

                    if t.len() <= 0 as usize{
                        return Err(CreatorError::LoginError)
                    }
                    let db_creator = &t[0];

                    
                    // Parse the id
                    let id = db_creator.id.to_owned();
                    let record_id = id.id.to_string();
                    let tb = id.tb.to_string();
                    let id = format!("{tb}:{record_id}");
                    println!("Record id :->  {id}");

                    if db_creator.login_attempts >= 10 as u8{ //Check if user has exceded the required amount of logins
                        eprintln!("Too many attempts");
                        println!("\n\n============================================================================================\n\n\n");
                        return Err(CreatorError::LoginAttemptsError) //else 
                    }
                    println!("cont....");
                    if creator.password == db_creator.password{ //Check if password is correct
                        info!("{:?} has logged in", &t[0].username);
                        
                        let now = Utc::now().timestamp() as usize; // UTC time to the number of seconds since the UNIX epoch (January 1, 1970, 00:00:00 UTC).
                        // TODO: Convert the id to string
                        let ret_crt = Claims{
                            id,
                            email: String::from(&db_creator.email),
                            username: String::from(&db_creator.username),
                            acc_type: String::from("creator"),
                            verified: db_creator.verified,
                            exp:now 
                        };// To be used to generate JWT token

                        let _query:Option<CreatorForLoginSuccess> = db.update(("creator", &record_id)).merge(json!({"login_attempts": 0})).await.unwrap(); // reset the number of login attempts

                        println!("\n\n============================================================================================\n\n\n");
                        return Ok(ret_crt); //return the Claim necessary for a cookie
                    }else{
                        print!("Wrong credentials");
                        
                        let query:Option<CreatorForLoginSuccess> = db.update(("creator", &record_id)).merge(json!({"login_attempts": &db_creator.login_attempts + 1})).await.unwrap();
                        dbg!("Update failed", query);
                        println!("\n\n============================================================================================\n\n\n");
                        return Err(CreatorError::WrongCredentialsError) //else
                    }
                },
                Err(e) => {
                    debug!("{e}");
                    println!("\n\n============================================================================================\n\n\n");
                    return Err(CreatorError::LoginError)
                }
            }      // INNER MATCH ---------------------------------------------------------------------------
        },
        Err(e) =>{
            error!("Creator logged in error");
            dbg!(e);
            println!("\n\n============================================================================================\n\n\n");
            return Err(CreatorError::LoginError)
            }
    }
    ;
}


pub async fn get_details(db: &Surreal<Client>, id: String) -> Result<Creator, CreatorError>{
    println!("\n\n{:<12}=====================================================================\n\n", "creator::details()");
    let id: &str= id.split(":").collect::<Vec<&str>>()[1];
    println!("{id}");
    let creator:Result<Option<Creator>, surrealdb::Error> = db.select(("creator", id)).await;

    match creator{
        Ok(c) => {
                // dbg!(&c);
                if let Some(ct) = c{
                    return Ok(ct)
                }else{
                    return Err(CreatorError::DetailsRetrievingError)
                }
        },
        Err(_) => {
            Err(CreatorError::DetailsRetrievingError)
        },
    }
}

pub async fn update_details<I: Serialize + for<'a> Deserialize<'a>>(db: &Surreal<Client>, id: String, field:&str, item: I) -> Result<bool, CreatorError>  {
    println!("\n\n{:<12}=====================================================================\n\n", "creator::details()");
    let id: &str = id.split(":").collect::<Vec<&str>>()[1];
    println!("{id}");
    //TODO: test if it accepts different types o
    let query:Option<CreatorForLoginSuccess> = db.update(("creator", field)).merge(json!({field: item})).await.unwrap(); // reset the number of login attempts
    return if let None = query {
        Err(CreatorError::DetailsUpdateError)
    } else if let Some(c) = query {
        Ok(true)
    } else {
        Err(CreatorError::DetailsUpdateError)
    }
}
//async fn get_details() -> Result<Vec<CreatorForCreate>, CreatorError>{}

// endsection:   -- controllers

// section:     -- imports
// endsection:   -- imports