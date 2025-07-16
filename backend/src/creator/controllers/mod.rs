use serde_json::json;
// section:     -- imports
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{
    creator::{CreatorDetailsList, CreatorError, CreatorForLoginSuccess},
    middleware::auth::jwt::Claims,
    studio::StudioFull,
    AvatarUrl,
};

use super::{
    CreatorDetails, CreatorForCreate, CreatorForLogin, CreatorForUpdateClient, CreatorForUpdateDb,
    Socials,
};
use chrono::Utc;
use tracing::{debug, error, info};
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
pub async fn register<'a>(
    db: &Surreal<Client>,
    creator: CreatorForCreate,
) -> Result<CreatorForCreate, CreatorError> {
    println!(
        "\n\n{:<12} ====================================================================\n\n",
        "creator::register()"
    );
    let created: Result<Option<CreatorForCreate>, surrealdb::Error> =
        db.create("creator").content(creator).await;

    match created {
        Ok(c) => {
            debug!("{:<12} - registered new creator", "FOR-DEV-ONLY");

            println!("\n\n============================================================================================\n\n");
            if let Some(g) = c {
                Ok(g)
            } else {
                Err(CreatorError::DetailsRetrievingError)
            }
        }
        Err(_e) => {
            error!("        - Creator registration error");
            println!("\n\n============================================================================================\n\n\n");
            Err(CreatorError::RegistrationError)
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
pub async fn login(db: &Surreal<Client>, creator: CreatorForLogin) -> Result<Claims, CreatorError> {
    //lequery
    println!(
        "\n\n{:<12}=====================================================================\n\n",
        "creator::login()"
    );
    let ct = db.
        query("SELECT id,email, role, password, username, verified, login_attempts FROM creator WHERE email = $email")
        .bind(("email", creator.email.clone()))
        .await;

    match ct {
        //We format the result for comparison and send back appropriate result
        Ok(mut c) => {
            debug!("{:<12} - creator login", "FOR-DEV-ONLY");

            #[allow(unused_mut)]
            let mut res: Result<Vec<CreatorForLoginSuccess>, surrealdb::Error> = c.take(0); // Convert the result to a vector containing the creator for login

            match res {
                //INNER MATCH ------------------------------------------------------------------
                Ok(t) => {
                    if t.len() <= 0_usize {
                        return Err(CreatorError::LoginError);
                    }
                    let db_creator = &t[0];

                    // Parse the id
                    let id = db_creator.id.to_owned();
                    let record_id = id.id.to_string();
                    let tb = id.tb.to_string();
                    let id = format!("{tb}:{record_id}");

                    if db_creator.login_attempts >= 10_u8 {
                        //Check if user has exceded the required amount of logins
                        eprintln!("Too many attempts");
                        println!("\n\n============================================================================================\n\n\n");
                        return Err(CreatorError::LoginAttemptsError); //else
                    }
                    if creator.password == db_creator.password {
                        //Check if password is correct
                        info!("{:?} has logged in", &t[0].username);

                        let now = Utc::now().timestamp() as usize; // UTC time to the number of seconds since the UNIX epoch (January 1, 1970, 00:00:00 UTC).
                                                                   // TODO: Convert the id to string
                        println!("{now}");
                        let ret_crt = Claims {
                            id,
                            email: db_creator.email.clone(),
                            username: db_creator.username.clone(),
                            acc_type: String::from("creator"),
                            verified: db_creator.verified,
                            exp: now,
                        }; // To be used to generate JWT token

                        let _query: Option<CreatorForLoginSuccess> = db
                            .update(("creator", &record_id))
                            .merge(json!({"login_attempts": 0}))
                            .await
                            .unwrap(); // reset the number of login attempts

                        println!("\n\n============================================================================================\n\n\n");
                        Ok(ret_crt) //return the Claim necessary for a cookie
                    } else {
                        print!("Wrong credentials");
                        // TODO:-> Give appropriate message for wrong credentials
                        let query: Option<CreatorForLoginSuccess> = db
                            .update(("creator", &record_id))
                            .merge(json!({"login_attempts": &db_creator.login_attempts + 1}))
                            .await
                            .unwrap();
                        dbg!("Update failed", query);
                        println!("\n\n============================================================================================\n\n\n");
                        Err(CreatorError::WrongCredentialsError) //else
                    }
                }
                Err(e) => {
                    debug!("{e}");
                    println!("\n\n============================================================================================\n\n\n");
                    Err(CreatorError::LoginError)
                }
            } // INNER MATCH ---------------------------------------------------------------------------
        }
        Err(e) => {
            error!("Creator logged in error");
            dbg!(e);
            println!("\n\n============================================================================================\n\n\n");
            Err(CreatorError::LoginError)
        }
    }
}

/// Used to get Cretors details
pub async fn get_details(db: &Surreal<Client>, id: String) -> Result<CreatorDetails, CreatorError> {
    println!(
        "\n\n{:<12}=====================================================================\n\n",
        "creator::details()"
    );
    let id: &str = id.split(":").collect::<Vec<&str>>()[1];
    let creator: Result<Option<CreatorDetails>, surrealdb::Error> =
        db.select(("creator", id)).await;

    match creator {
        Ok(c) => {
            // dbg!(&c);
            if let Some(ct) = c {
                Ok(ct)
            } else {
                Err(CreatorError::DetailsRetrievingError)
            }
        }
        Err(_) => Err(CreatorError::DetailsRetrievingError),
    }
}
///
pub async fn index(db: &Surreal<Client>) -> Result<Vec<CreatorDetailsList>, CreatorError> {
    println!(
        "\n\n{:<12}=====================================================================\n\n",
        "creator::index()"
    );
    let creator: Result<Vec<CreatorDetailsList>, surrealdb::Error> = db.select("creator").await;

    match creator {
        Ok(c) => {
            // dbg!(&c);
            Ok(c)
        }
        Err(e) => {
            dbg!(e);
            Err(CreatorError::DetailsRetrievingError)
        }
    }
}

#[allow(dead_code)]
pub async fn get_avatar_url(db: &Surreal<Client>, id: String) -> Result<AvatarUrl, CreatorError> {
    println!(
        "\n\n{:<12}=====================================================================\n\n",
        "creator::details()"
    );
    let id: &str = id.split(":").collect::<Vec<&str>>()[1];
    let creator: Result<Option<AvatarUrl>, surrealdb::Error> = db.select(("creator", id)).await;

    match creator {
        Ok(c) => {
            // dbg!(&c);
            if let Some(ct) = c {
                Ok(ct)
            } else {
                Err(CreatorError::DetailsRetrievingError)
            }
        }
        Err(_) => Err(CreatorError::DetailsRetrievingError),
    }
}

pub async fn update_details(
    db: &Surreal<Client>,
    id: String,
    payload: CreatorForUpdateClient,
) -> Result<CreatorForUpdateDb, CreatorError> {
    println!(
        "\n\n{:<12}=====================================================================\n\n",
        "creator::update_details()"
    );
    let id: &str = id.split(":").collect::<Vec<&str>>()[1];
    //TODO: test if it accepts different types o
    // Items to update username, password, socials, description,
    let mut query: Option<CreatorForUpdateDb> = None;
    dbg!("Payload: {}", &payload);
    let field = payload.field.clone();
    match field.as_str() {
        // Matching the field to restrict it to the user updatable  items named above
        "socials" => {
            println!("socials");
            // Socials is a struct so it needs special processing
            let c: Option<CreatorForUpdateDb> = db.select(("creator", id)).await.unwrap(); // Select user from db
            if let Some(c) = c {
                let socials_db = c.socials; // Get the socials object from db
                let value = payload.value.clone();
                let value: Vec<&str> = value.split(";").collect();
                println!("{:?}", &value);
                let mut socials: Socials;
                if let Some(s) = socials_db {
                    socials = s;
                } else {
                    socials = Socials::new();
                }
                match value[0] {
                    // Used to update the specific social media username
                    "facebook" => {
                        socials.facebook = Some(value[1].to_string());
                    }
                    "twitter_x" => {
                        socials.twitter_x = Some(value[1].to_string());
                    }
                    "instagram" => {
                        socials.instagram = Some(value[1].to_string());
                    }
                    &_ => {
                        return Err(CreatorError::DetailsUpdateError);
                    }
                }
                query = db
                    .update(("creator", id))
                    .merge(json!({"socials": socials}))
                    .await
                    .unwrap() // reset the number of login attempts
            }
        } //End of socials arm
        "username" | "description" | "avatar" => {
            query = db
                .update(("creator", id))
                .merge(json!({&payload.field: &payload.value}))
                .await
                .unwrap(); // reset the number of login attempts
        }
        "password" => {
            // Password is also special
            println!("IS IT THE BRAIDS") //TODO: Implement this
        }

        &_ => return Err(CreatorError::DetailsUpdateError),
    }

    if query.is_none() {
        Err(CreatorError::DetailsUpdateError)
    } else if let Some(c) = query {
        println!("{:?}", c);
        Ok(c)
    } else {
        Err(CreatorError::DetailsUpdateError)
    }
}

pub async fn delete_creator(
    db: &Surreal<Client>,
    id: String,
) -> Result<CreatorDetails, CreatorError> {
    println!(
        "\n\n{:<12}=====================================================================\n\n",
        "creator::details()"
    );
    let id: &str = id.split(":").collect::<Vec<&str>>()[1];
    //TODO: test if it accepts different types o
    // Items to update username, password, socials, description,
    let deleted_creator: Result<Option<CreatorDetails>, surrealdb::Error> =
        db.delete(("creator", id)).await;
    match deleted_creator {
        Ok(dc) => {
            if let Some(c) = dc {
                Ok(c)
            } else {
                Err(CreatorError::RegistrationError)
            }
        }
        Err(_) => Err(CreatorError::RegistrationError),
    }
}
//async fn get_details() -> Result<Vec<CreatorForCreate>, CreatorError>{}

pub async fn get_studios(
    db: &Surreal<Client>,
    id: String,
) -> Result<Vec<StudioFull>, CreatorError> {
    eprintln!("{id}");
    let query = format!("SELECT * FROM studio WHERE owner = {id};");
    debug!("{query}");
    let studios: Vec<StudioFull> = db.query(query).await?.take(0)?;

    Ok(studios)
}
// endsection:   -- controllers

// section:     -- imports
// endsection:   -- imports
