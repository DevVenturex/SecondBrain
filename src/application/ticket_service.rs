use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::{domain::Ticket, error::Error};

#[async_trait]
pub trait TicketService {
    async fn get_ticket_by_id(&self, id: i32) -> Result<Ticket, Error>;

    async fn list_tickets(&self) -> Result<Vec<Ticket>, Error>;

    async fn create_ticket(
        &self,
        title: String,
        description: Option<String>,
    ) -> Result<Ticket, Error>;

    async fn update_ticket(
        &self,
        id: i32,
        title: Option<String>,
        description: Option<String>,
    ) -> Result<Ticket, Error>;

    async fn delete_ticket(&self, id: i32) -> Result<Ticket, Error>;
}

#[derive(Clone)]
pub struct StoreTicketService {
    tickets: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl StoreTicketService {
    pub async fn new() -> Result<Self, Error> {
        Ok(Self {
            tickets: Arc::default(),
        })
    }
}

#[async_trait]
impl TicketService for StoreTicketService {
    async fn get_ticket_by_id(&self, id: i32) -> Result<Ticket, Error> {
        let store = self.tickets.lock().await;
        store
            .iter()
            .filter_map(|t| t.clone())
            .find(|ticket| ticket.id == id)
            .ok_or(Error::NotFound(format!("Ticket with id {id}")))
    }

    async fn list_tickets(&self) -> Result<Vec<Ticket>, Error> {
        let store = self.tickets.lock().await;
        Ok(store
            .iter()
            .filter_map(|t| t.clone())
            .collect::<Vec<Ticket>>())
    }

    async fn create_ticket(
        &self,
        title: String,
        description: Option<String>,
    ) -> Result<Ticket, Error> {
        let mut store = self.tickets.lock().await;
        let id = match store.iter().filter_map(|t| t.clone()).map(|t| t.id).max() {
            Some(id) => id + 1,
            None => 0,
        };
        let now = chrono::Utc::now();
        let ticket = Ticket {
            id,
            title,
            description: description.unwrap_or_default(),
            create_at: now,
            updated_at: now,
        };

        store.push(Some(ticket.clone()));
        Ok(ticket)
    }

    async fn update_ticket(
        &self,
        id: i32,
        title: Option<String>,
        description: Option<String>,
    ) -> Result<Ticket, Error> {
        let mut store = self.tickets.lock().await;
        let ticket = store
            .iter_mut()
            .filter_map(|t| t.as_mut())
            .find(|ticket| ticket.id == id)
            .ok_or(Error::NotFound(format!("Ticket with id {id}")))?;

        if let Some(title) = title {
            ticket.title = title;
        }
        if let Some(desc) = description {
            ticket.description = desc;
        }
        ticket.updated_at = chrono::Utc::now();
        Ok(ticket.clone())
    }

    async fn delete_ticket(&self, id: i32) -> Result<Ticket, Error> {
        let mut store = self.tickets.lock().await;
        let index = store
            .iter()
            .position(|opt| {
                opt.as_ref().is_some_and(|t| t.id == id)
            })
            .ok_or(Error::NotFound(format!("Ticket with id {id}")))?;

        let deleted = store[index]
            .take()
            .ok_or(Error::NotFound(format!("Ticket with id {id}")))?;
        Ok(deleted)
    }
}
