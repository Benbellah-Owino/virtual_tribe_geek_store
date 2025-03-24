use serde::{Serialize, Deserialize};
use surrealdb::sql::Thing;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SurrealID {
    pub id: Thing,
}
pub fn thing_from_string(id: String) -> Thing {
    let id_string: Vec<&str> = id.split(':').collect();
    // Thing {
    //     tb: id_string[0].to_string(),
    //     id: Id::from(id_string[1].to_string()),
    // }

    Thing::from((id_string[0].to_string(), id_string[1].to_string()))
}
