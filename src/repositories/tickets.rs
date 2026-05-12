#[cfg(test)]
use mockall::automock;

use surrealdb::{Connection, Surreal, types::RecordId};

use crate::{Error, models::ticket::*};

#[cfg_attr(test, automock)]
pub trait TicketRepositoryTrait {
    fn insert(&self, value: CreateTicket) -> impl Future<Output = Result<Record, Error>> + Send;
    fn find(&self, value: FindTicket) -> impl Future<Output = Result<Vec<Ticket>, Error>> + Send;
    fn update(&self, value: UpdateTicket) -> impl Future<Output = Result<Record, Error>> + Send;
    fn delete(&self, value: DeleteTicket) -> impl Future<Output = Result<Record, Error>> + Send;
    fn get_all(&self) -> impl Future<Output = Result<Vec<Ticket>, Error>> + Send;
}

#[derive(Debug, Clone)]
pub struct TicketRepository<C>
where 
    C: Connection,
{
    db: Surreal<C>,
    table: String,
}

impl<C: Connection> TicketRepository<C> {
    pub fn new(db: Surreal<C>) -> Self {
        Self {
            db,
            table: String::from("tickets"),
        }
    }
}

impl<C: Connection> TicketRepositoryTrait for TicketRepository<C> {
    fn insert(&self, value: CreateTicket) -> impl Future<Output = Result<Record, Error>> + Send {
        async move {
            let timestamp = chrono::Utc::now();
            let ticket = Ticket {
                title: value.title,
                description: value.description,
                created_at: timestamp,
                updated_at: timestamp,
            };
            let record = self
                .db
                .create(&self.table)
                .content(ticket)
                .await?
                .ok_or(Error::InternalError)?;
            Ok(record)
        }
    }

    fn find(&self, value: FindTicket) -> impl Future<Output = Result<Vec<Ticket>, Error>> + Send {
        async move {
            let record = self
                .db
                .select((&self.table, value.id.key))
                .await?
                .ok_or(Error::NotFound)?;
            Ok(record)
        }
    }

    fn update(&self, value: UpdateTicket) -> impl Future<Output = Result<Record, Error>> + Send {
        async move {
            let record = self
                .db
                .update((&self.table, value.clone().id.key))
                .content(value)
                .await?
                .ok_or(Error::NotFound)?;
            Ok(record)
        }
    }

    fn delete(&self, value: DeleteTicket) -> impl Future<Output = Result<Record, Error>> + Send {
        async move {
            let record = self
                .db
                .delete((&self.table, value.id.key))
                .await?
                .ok_or(Error::NotFound)?;
            Ok(record)
        }
    }

    fn get_all(&self) -> impl Future<Output = Result<Vec<Ticket>, Error>> + Send {
        async move {
            let records = self
                .db
                .select(&self.table)
                .await?;
            Ok(records)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::future;

    use mockall::predicate::eq;

    use super::*;

    #[tokio::test]
    async fn test_create_ticket() {
        let mut repo = MockTicketRepositoryTrait::new();
        repo.expect_insert()
            .with(eq(CreateTicket {
                title: String::from("Test Ticket"),
                description: None,
            }))
            .returning(|_| {
                Box::pin(future::ready(Ok(Ticket {
                    number: 0,
                    title: String::from("Test Ticket"),
                    description: None,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                })))
            });

        let result = repo
            .insert(CreateTicket {
                title: String::from("Test Ticket"),
                description: None,
            })
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_ticket_with_desc() {
        let mut repo = MockTicketRepositoryTrait::new();
        repo.expect_insert()
            .with(eq(CreateTicket {
                title: String::from("Test Ticket"),
                description: Some(String::from("Test description")),
            }))
            .returning(|_| {
                Box::pin(future::ready(Ok(Ticket {
                    number: 0,
                    title: String::from("Test Ticket"),
                    description: Some(String::from("Test description")),
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                })))
            });

        let result = repo
            .insert(CreateTicket {
                title: String::from("Test Ticket"),
                description: Some(String::from("Test description")),
            })
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_all_tickets() {
        let mut repo = MockTicketRepositoryTrait::new();
        repo.expect_get_all().returning(|| {
            Box::pin(future::ready(Ok(vec![
                Ticket {
                    number: 0,
                    title: String::from("Ticket 0"),
                    description: None,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                },
                Ticket {
                    number: 1,
                    title: String::from("Ticket 1"),
                    description: None,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                },
                Ticket {
                    number: 2,
                    title: String::from("Ticket 2"),
                    description: None,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                },
            ])))
        });

        let result = repo.get_all().await;
        assert!(result.is_ok());
        let result = result.ok().unwrap();
        assert_eq!(result.len(), 3);
    }

    #[tokio::test]
    async fn test_find_ticket() {
        let mut repo = MockTicketRepositoryTrait::new();
        repo.expect_find()
            .with(eq(FindTicket { number: 0 }))
            .returning(|_| {
                Box::pin(future::ready(Ok(vec![Ticket {
                    number: 0,
                    title: String::from("Test Ticket"),
                    description: Some(String::from("Test description")),
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                }])))
            });

        let result = repo.find(FindTicket { number: 0 }).await;
        assert!(result.is_ok());
        let result = result.ok().unwrap();
        assert_eq!(result.len(), 1);
    }

    #[tokio::test]
    async fn test_delete_ticket() {
        let mut repo = MockTicketRepositoryTrait::new();
        repo.expect_delete()
            .with(eq(DeleteTicket { number: 0 }))
            .returning(|_| {
                Box::pin(future::ready(Ok(Ticket {
                    number: 0,
                    title: String::from("Test Ticket"),
                    description: Some(String::from("Test description")),
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                })))
            });
        let result = repo.delete(DeleteTicket { number: 0 }).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_ticket() {
        let mut repo = MockTicketRepositoryTrait::new();
        repo.expect_update()
            .with(eq(UpdateTicket {
                number: 0,
                title: Some(String::from("New Title")),
                description: None,
            }))
            .returning(|_| {
                Box::pin(future::ready(Ok(Ticket {
                    number: 0,
                    title: String::from("New Title"),
                    description: Some(String::from("Test description")),
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                })))
            });
        let result = repo
            .update(UpdateTicket {
                number: 0,
                title: Some(String::from("New Title")),
                description: None,
            })
            .await;
        assert!(result.is_ok());
        let result = result.ok().unwrap();
        assert_eq!(result.title, String::from("New Title"));
    }
}
