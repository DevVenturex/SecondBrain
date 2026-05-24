use std::sync::Arc;

use axum::Router;
use second_brain::{
    api,
    app_state::{AppState, TicketState},
    application::StoreTicketService,
    error::Error,
};
use tokio::net::TcpListener;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .with_target(true)
                .with_current_span(true),
        )
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let ticket_service = StoreTicketService::new().await?;
    let app_state = AppState {
        tickets: TicketState::new(Arc::new(ticket_service)).await,
    };
    let app = Router::new()
        .merge(api::routes(app_state))
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new());

    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    axum::serve(listener, app).await?;
    Ok(())
}
