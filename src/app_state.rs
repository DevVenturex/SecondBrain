use std::sync::Arc;

use surrealdb::{
    Surreal,
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
};

use crate::{Error, repositories::tickets::TicketRepository};

#[derive(Debug, Clone)]
pub struct AppState {
    pub tickets: Arc<TicketRepository<Client>>,
}

impl AppState {
    pub async fn new(db_connection: &'static str) -> Result<Self, Error> {
        let db = Surreal::new::<Ws>(db_connection).await?;
        db.signin(Root {
            username: String::from("root"),
            password: String::from("Passw0rd!"),
        })
        .await?;

        db.use_ns("second_brain").use_db("test_tickets").await?;

        let ticket_repo = TicketRepository::new(db);
        Ok(Self {
            tickets: Arc::new(ticket_repo),
        })
    }
}
