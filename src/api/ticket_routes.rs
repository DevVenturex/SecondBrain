use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};

use crate::{
    app_state::{AppState, TicketState}, domain::{CreateTicket, Ticket, UpdateTicket}, error::Error
};

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route(
            "/tickets",
            get(get_all)
                .put(create)
                .patch(update)
        )
        .route("/tickets/{id}", axum::routing::delete(delete).post(get_by_id))
        .with_state(state)
}

async fn get_all(State(state): State<TicketState>) -> Result<Json<Vec<Ticket>>, Error> {
    match state.service.list_tickets().await {
        Ok(tickets) => Ok(Json(tickets)),
        Err(e) => Err(e),
    }
}

async fn get_by_id(
    State(state): State<TicketState>,
    Path(id): Path<i32>,
) -> Result<Json<Ticket>, Error> {
    match state.service.get_ticket_by_id(id).await {
        Ok(ticket) => Ok(Json(ticket)),
        Err(e) => Err(e),
    }
}

async fn create(
    State(state): State<TicketState>,
    Json(ticket): Json<CreateTicket>,
) -> Result<Json<Ticket>, Error> {
    match state.service
        .create_ticket(ticket.title, ticket.description)
        .await
    {
        Ok(ticket) => Ok(Json(ticket)),
        Err(e) => Err(e),
    }
}

async fn update(
    State(state): State<TicketState>,
    Json(ticket): Json<UpdateTicket>,
) -> Result<Json<Ticket>, Error> {
    match state.service
        .update_ticket(ticket.id, ticket.title, ticket.description)
        .await
    {
        Ok(ticket) => Ok(Json(ticket)),
        Err(e) => Err(e),
    }
}

async fn delete(
    State(state): State<TicketState>,
    Path(id): Path<i32>,
) -> Result<Json<Ticket>, Error> {
    match state.service
        .delete_ticket(id)
        .await
    {
        Ok(ticket) => Ok(Json(ticket)),
        Err(e) => Err(e),
    }
}
