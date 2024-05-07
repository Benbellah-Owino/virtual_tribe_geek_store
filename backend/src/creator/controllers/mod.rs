// section:     -- imports
use surrealdb::{
    engine::remote::ws::Client, Response, Surreal
};

use crate::creator::CreatorError;

use super::CreatorForCreate;
use tracing::{info, debug,error};

// endsection:   -- imports


// section:     -- mods
// endsection:   -- mods



// section:     -- controllers
pub async fn register<'a>(db: &Surreal<Client>,creator:CreatorForCreate) -> Result<Vec<CreatorForCreate>, CreatorError>{

println!("{:<12} --------------------------------------------------------------------\n\n", "register()");
    let created: Result<Vec<CreatorForCreate>,surrealdb::Error>= db.create("creator")
                                        .content(creator)
                                        .await;

    match created{
        Ok(c) =>{
            debug!("{:<12} - registered new creator", "FOR-DEV-ONLY");
            println!("--------------------------------------------------------------------\n\n");
            println!("{:?}", c);

    println!("\n\n============================================================================================\n\n");
            return Ok(c)
        },
        Err(e)=>{
            error!("        - Creator registration error");
            dbg!(e);
    println!("\n\n============================================================================================\n\n");
            return Err(CreatorError::RegistrationError)
        }
    } 
    

}

pub async fn login(db: Surreal<Client>,creator:CreatorForCreate) -> Result<Response, CreatorError> {
    let creator = db.
                                    query("SELECT email, role, password, username FROM creator WHERE email = type::email")
                                    .bind(("email",&creator.email))
                                    .await;

    match creator{
        Ok(c) =>{
            debug!("{:<12} - creator login", "FOR-DEV-ONLY");
            println!("{:?} logged in", c);
            return Ok(c)
        },
        Err(e) =>{
            debug!("         - Creator logged in error");
            dbg!(e);
            return Err(CreatorError::LoginError)

        }
    }
    ;
}

//async fn get_details() -> Result<Vec<CreatorForCreate>, CreatorError>{}

// endsection:   -- controllers


// section:     -- imports
// endsection:   -- imports