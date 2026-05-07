use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{models::ticket::*, repositories::Repository};

pub struct TicketRepository {
    tickets: Arc<Mutex<Vec<Option<Ticket>>>>
}

impl Repository<Ticket> for TicketRepository {
    async fn insert(&self, value: CreateTicket) -> Ticket {
        let mut store = self.tickets.lock().await;
        let time = chrono::Utc::now();
        let ticket = Ticket {
            id: 0,
            title: value.title.clone(),
            description: value.description,
            created_at: time,
            updated_at: time,
        };

        store.push(Some(ticket.clone()));
        ticket
    }
    
    async fn find(&self, value: FindTicket) -> Vec<Ticket> {
        let store = self.tickets.lock().await;
        let tickets: Vec<Ticket> = store
            .iter()
            .filter_map(Option::as_ref)
            .filter(|ticket| ticket.id == value.id)
            .cloned()
            .collect();
        tickets
    }
    
    async fn update(&self, value: UpdateTicket) -> Ticket {
        let store = self.tickets.lock().await;
        let mut ticket: Ticket = store
            .iter()
            .filter_map(Option::as_ref)
            .find(|t| t.id == value.id)
            .cloned()
            .unwrap();
        if let Some(title) = value.title {
            ticket.title = title;
        }

        ticket.description = value.description;
        let time = chrono::Utc::now();
        ticket.created_at = time;
        ticket.updated_at = time;
        ticket
    }
    
    async fn delete(&self, value: DeleteTicket) -> Ticket {
        let mut store = self.tickets.lock().await;
        store
            .iter_mut()
            .find(|t| {
                if let Some(ticket) = t {
                    return ticket.id == value.id;
                }
                return false;
            })
            .and_then(|t| t.take())
            .unwrap()
    }
    
    async fn get_all(&self) -> Vec<Ticket> {
        let store = self.tickets.lock().await;
        store.iter().filter_map(|t| t.clone()).collect()
    }
}