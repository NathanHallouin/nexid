use std::net::SocketAddr;
use std::sync::Arc;

use axum::Router;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod crypto;
mod db;
mod error;
mod handlers;
mod middleware;
mod models;
mod oauth2;
mod routes;

use crate::config::Config;
use crate::db::Database;

/// Application state shared across handlers
pub struct AppState {
    pub config: Config,
    pub db: Database,
    pub redis: redis::Client,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "nexid=debug,tower_http=debug,axum=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::load()?;
    tracing::info!("Starting NexID v{}", config.version);

    // Connect to database
    let db = Database::connect(&config.database_url).await?;
    tracing::info!("Connected to PostgreSQL");

    // Connect to Redis
    let redis = redis::Client::open(config.redis_url.as_str())?;
    tracing::info!("Connected to Redis");

    // Build application state
    let state = Arc::new(AppState { config: config.clone(), db, redis });

    // Build router
    let app = Router::new()
        .merge(routes::health_routes())
        .merge(routes::oauth2_routes(state.clone()))
        .merge(routes::api_v1_routes(state.clone()))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive()) // TODO: Configure properly for production
        .layer(middleware::security_headers());

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Listening on {}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    tracing::info!("Server stopped");
    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
    tracing::info!("Received shutdown signal");
}
