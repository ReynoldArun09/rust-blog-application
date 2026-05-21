mod config;
mod db;

use axum::{Router, routing::get};
use config::Config;
use dotenvy::dotenv;
use mongodb::Database;
use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Config,
    pub db: Database,
}

async fn health() -> &'static str {
    "Api is running"
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = Config::from_env();

    let db = db::connect(&config.mongodb_uri, &config.database_name)
        .await
        .expect("failed to connect to mongodb");

    let state = AppState {
        config: config.clone(),
        db,
    };

    let app = Router::new()
        .route("/health", get(health))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    println!("{} listening on http://{}", config.app_name, addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind listener");

    axum::serve(listener, app).await.expect("Server error");
}
