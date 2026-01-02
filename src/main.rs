mod api;
mod domain;
mod services;
mod db;
mod config;

use axum::{
    routing::get,
    Router,
};
use config::AppConfig;

#[tokio::main]
async fn main() {
    // Load configuration from .env, environment, and reference.toml
    let config = AppConfig::from_env_and_file().expect("Failed to load configuration");

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health", get(api::health));

    let addr = format!("127.0.0.1:{}", config.server_port);
    let listener = tokio::net::TcpListener::bind(&addr).await.expect("Failed to bind port");
    println!("Server running on http://{} (env: {}, db: {})", addr, config.environment, config.database_url);
    axum::serve(listener, app).await.unwrap();
}