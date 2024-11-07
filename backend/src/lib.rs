use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

pub mod content;
pub mod creator;
pub mod ctx;
pub mod dev_initial;
pub mod middleware;
pub mod studio;
pub mod user;
pub mod vrt_lib;
pub mod file_upload;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DbId {
    pub id: Thing,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AvatarUrl {
    pub avatar: Option<String>
}
