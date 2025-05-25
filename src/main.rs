mod config;
mod error;
mod handlers;
mod models;
mod services;

use std::net::SocketAddr;
use axum::{Router, routing::get};
use services::{AppState, BuddhaServiceImpl, DiscordNotifyServiceImpl};
use tower_http::{trace::TraceLayer, cors::CorsLayer};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = config::Config::new().expect("Failed to load configuration");
    let addr: SocketAddr = config.server_addr().parse().expect("Invalid server address");

    // Initialize services
    let buddha_service = BuddhaServiceImpl::new(config.clone());
    let discord_service = DiscordNotifyServiceImpl::new(config.clone());

    // Create app state
    let state = AppState::new(config, buddha_service, discord_service);

    // Build our application with routes
    let app = Router::new()
        .route("/health", get(handlers::health_check))
        .route("/buddha/notify", get(handlers::notify_handler))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state);

    // Run our application
    info!("🚀 Server starting on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}