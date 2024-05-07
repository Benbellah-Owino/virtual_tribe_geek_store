// section:     -- imports
use surrealdb::{
    engine::remote::ws::Client, Response, Surreal
};

use crate::creator::CreatorError;

use super::{Creator, CreatorForCreate, CreatorForLogin};
use tracing::{ info, debug,error};

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

pub async fn login(db: &Surreal<Client>,creator:CreatorForLogin) -> Result<Creator, CreatorError> {
    //lequery
    let c:Option<CreatorForLogin> = db.select(("creator", &creator.email)).await.unwrap();
    dbg!("{c}",c );
    let ct = db.
                                    query("SELECT email, role, password, username FROM creator WHERE email = $email")
                                    .bind(("email",&creator.email))
                                    .await; 


    match ct{ //We format the result for comparison and send back appropriate result 
        Ok(mut c) =>{
            debug!("{:<12} - creator login", "FOR-DEV-ONLY");
            #[allow(unused_mut)]
            let mut res: Result<Vec<CreatorForLogin>, surrealdb::Error> = c.take(0); // Convert the result to a vector containing the creator for login
        
            match res{ //INNER MATCH ------------------------------------------------------------------
                Ok(t) => {
                    let db_creator = &t[0];

                    if creator.password == db_creator.password{ //Check if password is correct
                        info!("{:?} has logged in", t[0]);
                        //TODO: Fix C
                        let ret_crt:Vec<Creator> =  c.take(0).unwrap(); //C is empty at this point TODO: Fix 
                        println!("{:?}", ret_crt);
                        let ret_crt = ret_crt[0].clone(); 
                        return Ok(ret_crt); //return the full creator details
                    }else{
                        print!("Wrong credentials");
                        return Err(CreatorError::LoginError) //else
                    }
                },
                Err(e) => {
                    debug!("{e}");
                    return Err(CreatorError::LoginError)
                }
            }      // INNER MATCH ---------------------------------------------------------------------------
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