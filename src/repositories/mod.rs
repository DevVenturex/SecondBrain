use crate::models::Model;

pub mod tickets;

pub trait Repository<M>
where 
    M: Model,
{
    async fn insert(&self, value: M::Insert) -> M;
    async fn find(&self, value: M::Find) -> Vec<M>;
    async fn update(&self, value: M::Update) -> M;
    async fn delete(&self, value: M::Delete) -> M;
    async fn get_all(&self) -> Vec<M>;
}