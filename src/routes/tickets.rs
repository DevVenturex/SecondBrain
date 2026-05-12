use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{get, post},
};

use crate::{
    AppState, Error,
    models::ticket::{CreateTicket, DeleteTicket, FindTicket, Record, Ticket, UpdateTicket},
    repositories::tickets::TicketRepositoryTrait,
};

pub fn ticket_routes(app_state: AppState) -> Router {
    Router::new()
        .route("/tickets", get(get_all_tickets).post(create_ticket).put(update_ticket).delete(delete_ticket))
        .route("/tickets/{ticket}", post(find_ticket))
        .with_state(app_state)
}

async fn get_all_tickets(State(app_state): State<AppState>) -> Result<Json<Vec<Ticket>>, Error> {
    let tickets = app_state.tickets.get_all().await?;
    Ok(Json(tickets))
}

async fn find_ticket(
    State(app_state): State<AppState>,
    Path(ticket): Path<FindTicket>,
) -> Result<Json<Vec<Ticket>>, Error> {
    let ticket = app_state.tickets.find(ticket).await?;
    Ok(Json(ticket))
}

async fn create_ticket(
    State(app_state): State<AppState>,
    Json(ticket): Json<CreateTicket>,
) -> Result<Json<Record>, Error> {
    let ticket = app_state.tickets.insert(ticket).await?;
    Ok(Json(ticket))
}

async fn update_ticket(
    State(app_state): State<AppState>,
    Json(ticket): Json<UpdateTicket>,
) -> Result<Json<Record>, Error> {
    let ticket = app_state.tickets.update(ticket).await?;
    Ok(Json(ticket))
}

async fn delete_ticket(
    State(app_state): State<AppState>,
    Query(ticket): Query<DeleteTicket>,
) -> Result<Json<Record>, Error> {
    let ticket = app_state.tickets.delete(ticket).await?;
    Ok(Json(ticket))
}
