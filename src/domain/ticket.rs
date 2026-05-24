use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Ticket {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub create_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateTicket {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateTicket {
    pub id: i32,
    pub title: Option<String>,
    pub description: Option<String>,
}