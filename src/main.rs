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
use db::{create_pool, PgPool};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::http::StatusCode;
use api::openapi::ApiDoc;
use utoipa::OpenApi;

#[tokio::main]
async fn main() {
    // Load configuration from .env, environment, and reference.toml
    let config = AppConfig::from_env_and_file().expect("Failed to load configuration");

    // Initialize DB pool and verify connection
    let pool = create_pool(&config.database_url)
        .await
        .expect("Failed to connect to the database");

    let openapi_json = ApiDoc::openapi().to_json().expect("Failed to serialize OpenAPI spec");
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health", get(api::health))
        .route("/openapi.json", get(|| async move {
            (StatusCode::OK, [("content-type", "application/json")], openapi_json.clone())
        }))
        .with_state(pool.clone());

    let addr = format!("127.0.0.1:{}", config.server_port);
    let listener = tokio::net::TcpListener::bind(&addr).await.expect("Failed to bind port");
    println!("Server running on http://{} (env: {}, db: {})", addr, config.environment, config.database_url);
    axum::serve(listener, app).await.unwrap();
}