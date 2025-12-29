mod error;
mod extractors;
mod handlers;
mod models;
mod repository;
mod routes;
mod service;
mod state;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use state::AppState;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file (similar to application.properties)
    // Use .env.dev, .env.prod for environment-specific config
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Load configuration from environment
    let mongodb_uri =
        std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    // Initialize state
    let state = AppState::new(&mongodb_uri)
        .await
        .expect("Failed to initialize application state");
    let state = web::Data::new(state);

    tracing::info!("Server running on http://{}:{}", host, port);

    // Build and run server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(TracingLogger::default())
            .wrap(cors)
            .app_data(state.clone())
            .configure(routes::configure)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
