use crate::models::Model;

pub mod tickets;

pub trait Repository<M>: Send + Sync
where 
    M: Model + Send + Sync,
{
    fn insert(&self, value: M::Insert) -> impl Future<Output = Option<M>> + Send;
    fn find(&self, value: M::Find) -> impl Future<Output = Vec<M>> + Send;
    fn update(&self, value: M::Update) -> impl Future<Output = Option<M>> + Send;
    fn delete(&self, value: M::Delete) -> impl Future<Output = Option<M>> + Send;
    fn get_all(&self) -> impl Future<Output = Vec<M>> + Send;
}