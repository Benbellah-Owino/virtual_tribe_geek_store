#[allow(unused_imports)]
use axum::middleware;
use axum::{routing::get, Router};
use axum_prometheus::PrometheusMetricLayer;
use backend::content::routers::content_router;
use backend::creator::routers::creator_router;
use backend::middleware::auth::cookies::list_cookies;
use backend::studio::routers::studio_router;
use backend::user::routers::user_router;
use backend::middleware::dev::log_request;
use backend::dev_initial::db::{connect_db, init_queries};

use http::header::CONTENT_TYPE;
#[allow(unused_imports)]
use surrealdb::engine::remote::ws::Ws;
use tower_cookies::CookieManagerLayer;
use http::Method;
use tower_http::cors::CorsLayer;
use tower::ServiceBuilder;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // \\#region Setup
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed tracing");

    // \\#region Setup

    // section:     -- Database
    let db = connect_db().await.unwrap();

    init_queries(&db).await?;
    // endsection:  -- Database

    // section:     -- Server setup

    // Metrics
    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();
    // routers
    let origins = ["http://localhost:5173".parse().unwrap()];

    let cors = CorsLayer::new()
        .allow_headers([CONTENT_TYPE])
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_origin(origins);

    let app = Router::new()
        .route("/", get(|| async { "Hello, world" }))
        .layer(ServiceBuilder::new().layer(middleware::from_fn(list_cookies)))
        .nest("/content", content_router())
        .nest("/studio", studio_router())
        .nest("/user", user_router())
        .nest("/creator", creator_router())
        .layer(CookieManagerLayer::new())
        .layer(ServiceBuilder::new().layer(middleware::from_fn(log_request)))
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .layer(prometheus_layer)
        .layer(cors)
        .with_state(Ok(db));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:7878").await.unwrap();
    info!("Listening on port 7878...");
    axum::serve(listener, app).await.unwrap();

    // endsection:   -- Setup
    Ok(())
}
