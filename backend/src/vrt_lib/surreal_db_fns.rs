use surrealdb::sql::Thing;
use surrealdb::sql::Id;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use crate::studio::Studio;
use crate::studio::StudioError;
pub  enum Err{
    WrongFormat
}
pub fn str_to_thing(str_id: String) -> Result<Thing, Err>{
    if !str_id.contains(":"){
        return Err(Err::WrongFormat)
    }
    let id_string: Vec<String> = str_id.split(':').map(|s| s.to_string()).collect();
    return Ok( Thing {
        tb: id_string[0].to_owned(),
        id: Id::from(id_string[1].to_owned())
    })
}




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
    let studio: Result<Option<Studio>, surrealdb::Error> =
        db.select(("studio", id)).await;
    match studio {
        Ok(st) => {
            dbg!(&st);
            if let Some(s) = st{
                //TODO: FIX THIS
                let creator: Vec<String> = creator.split(':').map(|s|s.to_string()).collect();

                if &s.owner.id.to_string() == &creator[1] && &s.owner.tb == &creator[0]{
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
