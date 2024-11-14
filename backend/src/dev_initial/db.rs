use dotenv::dotenv;
use std::{env, fs};
use surrealdb;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use tracing::info;

#[derive(Debug, Clone)]
pub enum Error {
    DbConnectionError,
}

pub type Db = Result<Surreal<Client>, Error>;

// #[derive(Clone)]
// pub struct State{
//     db: Surreal<Client>
// }

pub async fn connect_db() -> Db {
    dotenv().ok();
    info!("Connecting to db INFO");

    let db = Surreal::new::<Ws>("127.0.0.1:8000").await;

    let db = db.unwrap();

    db.signin(Root {
        username: &env::var("DB_USERNAME").expect("Set db username"),
        password: &env::var("DB_PASSWORD").expect("Set db password"),
    })
    .await
    .unwrap();

    db.use_ns("virtual_tribe")
        .use_db("geek_store")
        .await
        .unwrap();
    info!("Connected to db");
    Ok(db)
}

pub async fn init_queries(db: &Surreal<Client>) -> surrealdb::Result<()> {
    let sql_setup: &str = "001_init.surql";
    let sql_dir: &str = "sql";

    let current_dir = std::env::current_dir().unwrap();
    println!("{:?}\n\n------------------------------", current_dir);
    let dir = current_dir.to_str().unwrap();
    let file_path: String = format!("{dir}\\{sql_dir}\\{sql_setup}");
    println!("{:?}\n\n------------------------------", file_path);

    let contents = fs::read_to_string(&file_path);
    match contents {
        Ok(t) => {
            let mut count: u8 = 1;
            let t_vec: Vec<_> = t.split(";").collect();
            println!("{:?}", t_vec);
            info!("Running queries");
            for i in t_vec {
                if i.contains("--") || i.is_empty() {
                    continue;
                } else {
                    let q = i.to_string();
                    println!("\n\n------------------------------");
                    println!("Query :-> {count} \n {q}");
                    let query = db.query(&q).await;
                    match query {
                        Ok(r) => {
                            println!("{:?}", r);
                        }
                        Err(e) => {
                            println!("\nError");
                            eprintln!("{:?}", e);
                        }
                    }
                    println!("\n\n------------------------------");
                    count += 1;
                }
            }
        }
        Err(e) => {
            println!("{:?}", e)
        }
    }

    Ok(())
}
