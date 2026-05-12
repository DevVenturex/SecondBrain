use second_brain::{
    Error,
    models::ticket::{CreateTicket, DeleteTicket},
    repositories::tickets::{TicketRepository, TicketRepositoryTrait},
};
use surrealdb::{
    Surreal,
    engine::local::{Db, Mem},
};

async fn setup_db() -> Result<Surreal<Db>, Error> {
    let db = Surreal::new::<Mem>(()).await?;
    db.use_ns("test").use_db("test").await?;
    Ok(db)
}

#[tokio::test]
async fn test_create_ticket() -> Result<(), Error> {
    let db = setup_db().await?;
    let repo = TicketRepository::new(db);

    let result = repo
        .insert(CreateTicket {
            title: String::from("Test Ticket"),
            description: None,
        })
        .await?;
    assert_eq!(result.title, String::from("Test Ticket"));

    Ok(())
}

// TODO: Fix NotFound error
#[tokio::test]
async fn test_delete_ticket() -> Result<(), Error> {
    let db = setup_db().await?;
    let repo = TicketRepository::new(db);
    let _ = repo.insert(CreateTicket { title: String::from("Delete this ticket"), description: None }).await?;
    let result = repo.delete(DeleteTicket { number: TicketNumber(0) }).await?;
    assert_eq!(result.title, String::from("Delete this ticket"));
    Ok(())
}

#[tokio::test]
async fn test_get_all_tickets() -> Result<(), Error> {
    let db = setup_db().await?;
    let repo = TicketRepository::new(db);
    repo.insert(CreateTicket { title: String::from("Ticket 0"), description: None }).await?;
    repo.insert(CreateTicket { title: String::from("Ticket 1"), description: None }).await?;
    repo.insert(CreateTicket { title: String::from("Ticket 2"), description: None }).await?;
    repo.insert(CreateTicket { title: String::from("Ticket 3"), description: None }).await?;
    let result = repo.get_all().await?;
    assert_eq!(result.len(), 4);
    Ok(())
}
