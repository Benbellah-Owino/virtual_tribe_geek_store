
use serde_json::json;
// section:     -- imports
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{
    user::{User, UserError, UserForLoginSuccess},
    middleware::auth::jwt::Claims,
};

use super::{
    UserForCreate, UserForLogin, UserForUpdateClient, UserForUpdateDb, 
};
use chrono::Utc;
use tracing::{debug, error, info};
// endsection:   -- imports

// section:     -- mods
// endsection:   -- mods

// section:     -- controllers

/// Adds a new User to the database
///
/// # Examples
///
/// ```
/// #[ignore]
/// async fn register(db:&Surreal<Client>){
/// let User = UserForCreate{
///        username: String::from("TestUser"),
///        email: String::from("testUser@gmail.com"),
///        role: vec![String::from("writer")],
///        password: String::from("password"),
///  };
/// let new_User = User::controllers::register(&db, User);
/// let t = register(&db, creater).await.unwrap();
/// let t = &t[0];
/// let _delete:Result<Vec<UserForCreate>, surrealdb::Error> = db.delete("User").await;
///
/// assert_eq!(String::from("TestUser"), t.username);    
/// assert_eq!(String::from("testUser@gmail.com"), t.email);    
/// assert_eq!(vec![String::from("writer")], t.role);    
/// assert_eq!(String::from("password"), t.password);
///}
/// ```
pub async fn register<'a>(
    db: &Surreal<Client>,
    user: UserForCreate,
) -> Result<Vec<UserForCreate>, UserError> {
    println!(
        "\n\n{:<12} ====================================================================\n\n",
        "User::register()"
    );
    let created: Result<Vec<UserForCreate>, surrealdb::Error> =
        db.create("user").content(user).await;

    match created {
        Ok(c) => {
            debug!("{:<12} - registered new User", "FOR-DEV-ONLY");

            println!("\n\n============================================================================================\n\n");
            return Ok(c);
        }
        Err(_e) => {
            error!("        - User registration error");
            println!("\n\n============================================================================================\n\n\n");
            return Err(UserError::RegistrationError);
        }
    }
}

/// Logs in User and returns a Claims struct for tokenization
///
/// # Examples
///
/// ```
/// #[ignore]
///async fn login(db:&Surreal<Client>,User: _){
/// let User = UserForLogin{
///        email: String::from("testUser@gmail.com"),
///        password: String::from("password"),
/// };
///
/// let logged = User::controllers::login(&db, User);
/// let claim = login(&db, User).await.unwrap();
///
/// let _delete:Result<Vec<User>, surrealdb::Error> = db.delete("User").await;
///
/// assert_eq!(String::from("TestUser"), t.username);    
/// assert_eq!(String::from("testUser@gmail.com"), t.email);    
/// assert_eq!(vec![String::from("User")], t.acc_type);    
/// assert_eq!(false, t.verified);
/// }
///
/// ```
pub async fn login(db: &Surreal<Client>, user: UserForLogin) -> Result<Claims, UserError> {
    //lequery
    println!(
        "\n\n{:<12}=====================================================================\n\n",
        "User::login()"
    );
    dbg!(&user);
    let ct = db.
        query("SELECT id,email, role, password, username, verified, login_attempts FROM user WHERE email = $email")
        .bind(("email", &user.email))
        .await;

    match ct {
        //We format the result for comparison and send back appropriate result
        Ok(mut c) => {
            debug!("{:<12} - user login", "FOR-DEV-ONLY");

            #[allow(unused_mut)]
            let mut res: Result<Vec<UserForLoginSuccess>, surrealdb::Error> = c.take(0); // Convert the result to a vector containing the User for login

            match res {
                //INNER MATCH ------------------------------------------------------------------
                Ok(t) => {
                    if t.len() <= 0 as usize {
                        return Err(UserError::LoginError);
                    }
                    let db_user = &t[0];

                    // Parse the id
                    let id = db_user.id.to_owned();
                    let record_id = id.id.to_string();
                    let tb = id.tb.to_string();
                    let id = format!("{tb}:{record_id}");

                    if db_user.login_attempts >= 10 as u8 {
                        //Check if user has exceded the required amount of logins
                        eprintln!("Too many attempts");
                        println!("\n\n============================================================================================\n\n\n");
                        return Err(UserError::LoginAttemptsError); //else
                    }
                    println!("cont....");
                    if user.password == db_user.password {
                        //Check if password is correct
                        info!("{:?} has logged in", &t[0].username);

                        let now = Utc::now().timestamp() as usize; // UTC time to the number of seconds since the UNIX epoch (January 1, 1970, 00:00:00 UTC).
                                                                   // TODO: Convert the id to string
                        let ret_crt = Claims {
                            id,
                            email: String::from(&db_user.email),
                            username: String::from(&db_user.username),
                            acc_type: String::from("user"),
                            exp: now,
                            verified: db_user.verified,
                        }; // To be used to generate JWT token

                        let _query: Option<UserForLoginSuccess> = db
                            .update(("user", &record_id))
                            .merge(json!({"login_attempts": 0}))
                            .await
                            .unwrap(); // reset the number of login attempts

                        println!("\n\n============================================================================================\n\n\n");
                        return Ok(ret_crt); //return the Claim necessary for a cookie
                    } else {
                        print!("Wrong credentials");

                        let query: Option<UserForLoginSuccess> = db
                            .update(("user", &record_id))
                            .merge(json!({"login_attempts": &db_user.login_attempts + 1}))
                            .await
                            .unwrap();
                        dbg!("Update failed", query);
                        println!("\n\n============================================================================================\n\n\n");
                        return Err(UserError::WrongCredentialsError); //else
                    }
                }
                Err(e) => {
                    debug!("{e}");
                    println!("\n\n============================================================================================\n\n\n");
                    return Err(UserError::LoginError);
                }
            } // INNER MATCH ---------------------------------------------------------------------------
        }
        Err(e) => {
            error!("User logged in error");
            dbg!(e);
            println!("\n\n============================================================================================\n\n\n");
            return Err(UserError::LoginError);
        }
    };
}

/// Used to get Cretors details
pub async fn get_details(db: &Surreal<Client>, id: String) -> Result<User, UserError> {
    println!(
        "\n\n{:<12}=====================================================================\n\n",
        "User::details()"
    );
    let id: &str = id.split(":").collect::<Vec<&str>>()[1];
    let user: Result<Option<User>, surrealdb::Error> = db.select(("user", id)).await;

    match user {
        Ok(c) => {
            dbg!(&c);

            if let Some(ct) = c {
                return Ok(ct);
            } else {
                return Err(UserError::DetailsRetrievingError);
            }
        }
        Err(_) => Err(UserError::DetailsRetrievingError),
    }
}

pub async fn update_details(
    db: &Surreal<Client>,
    id: String,
    payload: UserForUpdateClient,
) -> Result< UserForUpdateDb, UserError> {
    println!(
        "\n\n{:<12}=====================================================================\n\n",
        "User::details()"
    );
    let id: &str = id.split(":").collect::<Vec<&str>>()[1];
    //TODO: test if it accepts different types o
    // Items to update username, password, socials, description,
    let mut query: Option<UserForUpdateDb> = None;
    let field = payload.field.clone();

    match field.as_str() {
        // Matching the field to restrict it to the user updatable  items named above
        "username" | "description" => {
            query = db
                .update(("user", id))
                .merge(json!({&payload.field: &payload.value}))
                .await
                .unwrap(); // reset the number of login attempts

        }
        "password" => {
            // Password is also special
            println!("IS IT THE BRAIDS") //TODO: Implement this
        }
        &_ => return Err(UserError::DetailsUpdateError),
    }

    return if let None = query {
        Err(UserError::DetailsUpdateError)
    } else if let Some(c) = query {
        Ok(c)
    } else {
        Err(UserError::DetailsUpdateError)
    };
}

pub async fn delete_user(db: &Surreal<Client>, id: String) -> Result<User, UserError> {
    println!(
        "\n\n{:<12}=====================================================================\n\n",
        "User::details()"
    );
    let id: &str = id.split(":").collect::<Vec<&str>>()[1];
    //TODO: test if it accepts different types o
    // Items to update username, password, socials, description,
    let deleted_user: Result<Option<User>, surrealdb::Error> =
        db.delete(("user", id)).await;
    match deleted_user {
        Ok(dc) => {
            if let Some(c) = dc {
                return Ok(c);
            } else {
                return Err(UserError::RegistrationError);
            }
        }
        Err(_) => return Err(UserError::RegistrationError),
    };
}
//async fn get_details() -> Result<Vec<UserForCreate>, UserError>{}

// endsection:   -- controllers

// section:     -- imports
// endsection:   -- imports
