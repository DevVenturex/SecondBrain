use axum::Router;

use crate::{AppState, tickets::ticket_routes};

pub mod tickets;

pub fn routes(app_state: AppState) -> Router {
    Router::new()
        .nest("/api", ticket_routes(app_state))
}