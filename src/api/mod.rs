use axum::Router;

use crate::app_state::AppState;

mod ticket_routes;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .nest("/api", ticket_routes::routes(state))

}
