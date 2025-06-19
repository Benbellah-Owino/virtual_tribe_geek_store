use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

pub mod content;
pub mod creator;
pub mod ctx;
pub mod dev_initial;
pub mod file_upload;
pub mod helpers;
pub mod middleware;
pub mod studio;
pub mod test_grounds;
pub mod user;
pub mod vrt_lib;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DbId {
    pub id: Thing,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComicId {
    pub comic: Thing,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AvatarUrl {
    pub avatar: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Count {
    pub count: u32,
}
