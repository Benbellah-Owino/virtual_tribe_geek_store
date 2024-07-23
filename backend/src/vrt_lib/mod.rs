use serde::{Serialize, Deserialize};
use surrealdb::sql::Thing;

pub mod surreal_db_fns;



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemId {
    pub id: Thing,
}