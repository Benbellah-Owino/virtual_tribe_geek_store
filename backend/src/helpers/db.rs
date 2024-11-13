use surrealdb::sql::{Id, Thing};

pub fn thing_from_string(id: String) -> Thing {
    let id_string: Vec<&str> = id.split(':').collect();

    Thing {
        tb: id_string[0].to_string(),
        id: Id::from(id_string[1].to_string()),
    }
}
