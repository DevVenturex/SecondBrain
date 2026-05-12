use chrono::Utc;
use serde::{Deserialize, Serialize};
use surrealdb::types::{RecordId, SurrealValue, Value};

#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
pub struct Record {
    pub id: RecordId,
}

#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
pub struct Ticket {
    pub title: String,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateTicket {
    pub title: String,
    pub description: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FindTicket {
    pub id: RecordId,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize, PartialEq)]
pub struct UpdateTicket {
    pub id: RecordId,
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteTicket {
    pub id: RecordId,
}