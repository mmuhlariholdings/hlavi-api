mod handlers;
mod models;

use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "hlavi_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build the application router
    let app = Router::new()
        .route("/", get(handlers::health::health_check))
        .route("/health", get(handlers::health::health_check))
        .route("/api/v1/tickets", get(handlers::tickets::list_tickets))
        .route("/api/v1/tickets", post(handlers::tickets::create_ticket))
        .route(
            "/api/v1/tickets/:id",
            get(handlers::tickets::get_ticket)
                .put(handlers::tickets::update_ticket)
                .delete(handlers::tickets::delete_ticket),
        )
        .route("/api/v1/board", get(handlers::board::get_board))
        .layer(cors);

    // Start the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Hlavi API server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
