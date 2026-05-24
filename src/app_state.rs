use std::sync::Arc;


use axum::extract::FromRef;

use crate::application::TicketService;

#[derive(Clone)]
pub struct AppState {
    pub tickets: TicketState,
}

#[derive(Clone)]
pub struct TicketState {
    pub service: Arc<dyn TicketService + Send + Sync>
}

impl TicketState {
    pub async fn new(service: Arc<dyn TicketService + Send + Sync>) -> Self {
        Self {
            service
        }
    }
}

impl FromRef<AppState> for TicketState {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.tickets.clone()
    }
}