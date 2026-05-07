use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::models::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticket {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

impl Model for Ticket {
    type Insert = CreateTicket;

    type Find = FindTicket;

    type Update = UpdateTicket;

    type Delete = DeleteTicket;
}

#[derive(Debug)]
pub struct CreateTicket {
    pub title: String,
    pub description: Option<String>,
}

impl Insertable for CreateTicket {}

#[derive(Debug)]
pub struct FindTicket {
    pub id: u64,
}

impl Findable for FindTicket {}

#[derive(Debug)]
pub struct UpdateTicket {
    pub id: u64,
    pub title: Option<String>,
    pub description: Option<String>,
}

impl Updatable for UpdateTicket {}

#[derive(Debug)]
pub struct DeleteTicket {
    pub id: u64,
}

impl Deletable for DeleteTicket {}
