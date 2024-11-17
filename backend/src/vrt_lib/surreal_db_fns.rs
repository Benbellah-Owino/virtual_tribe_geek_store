use crate::studio::Studio;
use crate::studio::StudioError;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
pub enum Err {
    WrongFormat,
}

// TODO: Move to helpers
pub async fn check_owner_2(
    creator: &String,
    studio: &String,
    db: &Surreal<Client>,
) -> Result<Studio, StudioError> {
    // let query = db
    //     .query("SELECT * FROM studio WHERE id = $id")
    //     .bind(("id", studio))
    //     .await;
    println!("\n\n studio id -> {studio}");
    let id: &str = studio.split(":").collect::<Vec<&str>>()[1];
    let studio: Result<Option<Studio>, surrealdb::Error> = db.select(("studio", id)).await;
    match studio {
        Ok(st) => {
            dbg!(&st);
            if let Some(s) = st {
                //TODO: FIX THIS
                let creator: Vec<String> = creator.split(':').map(|s| s.to_string()).collect();

                if &s.owner.id.to_string() == &creator[1] && &s.owner.tb == &creator[0] {
                    Ok(s)
                } else {
                    Err(StudioError::OwnerMismatch)
                }
            } else {
                Err(StudioError::StudioRetrievingError)
            }
        }
        Err(_) => Err(StudioError::StudioRetrievingError),
    }
}
