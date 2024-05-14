#[allow(unused_imports)]
use axum::{routing::get, Router};
use backend::creator;

use backend::dev_initial::db::{connect_db, init_queries};

#[allow(unused_imports)]
use surrealdb::engine::remote::ws::Ws;
use tower_cookies::CookieManagerLayer;
// use surrealdb::opt::auth::Root;
// use surrealdb::sql::Thing;
// use surrealdb::Surreal;
use tracing::info;
use tracing_subscriber::FmtSubscriber;
#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // \\#region Setup
    let subscriber = FmtSubscriber::builder().finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed tracing");

    // \\#region Setup

    // section:     -- Database
    let db = connect_db().await.unwrap();

    init_queries(&db).await?;
    // endsection:  -- Database

    // section:     -- Server setup

    // routers
    let app = Router::new()
        .route("/", get(|| async { "Hello, world" }))
        .nest("/creator", creator::routers::creator_router())
        .layer(CookieManagerLayer::new())
        .with_state(Ok(db));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:7878").await.unwrap();
    info!("Listening on port 7878...");
    axum::serve(listener, app).await.unwrap();

    // endsection:   -- Setup
    Ok(())
}
