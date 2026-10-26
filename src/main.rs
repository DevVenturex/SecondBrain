use std::net::SocketAddr;

use axum::{Router, routing::get};
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("debug"))
                .unwrap(),
            )
            .init();
    let app = Router::new()
        .route("/", get(|| async { "Hello World"}))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener  = tokio::net::TcpListener::bind(addr).await?;
    info!("Server runs on {}", addr);

    axum::serve(listener, app).await?;
    Ok(())
}
