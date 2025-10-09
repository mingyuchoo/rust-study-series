use super::entities::Contact;
use super::errors::ContactResult;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait ContactRepository: Send + Sync {
    async fn create(&self, contact: Contact) -> ContactResult<Contact>;
    async fn get_by_id(&self, id: Uuid) -> ContactResult<Contact>;
    async fn get_all(&self) -> ContactResult<Vec<Contact>>;
    async fn update(&self, contact: Contact) -> ContactResult<Contact>;
    async fn delete(&self, id: Uuid) -> ContactResult<()>;
    async fn search(&self, query: &str) -> ContactResult<Vec<Contact>>;
}
