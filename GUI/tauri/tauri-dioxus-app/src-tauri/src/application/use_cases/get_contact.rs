use crate::domain::{Contact, ContactRepository, DomainError};
use std::sync::Arc;
use uuid::Uuid;

pub struct GetContactUseCase {
    repository: Arc<dyn ContactRepository>,
}

impl GetContactUseCase {
    pub fn new(repository: Arc<dyn ContactRepository>) -> Self {
        Self {
            repository,
        }
    }

    pub async fn execute(&self, id: Uuid) -> Result<Contact, DomainError> {
        match self.repository.get_by_id(id).await? {
            | Some(contact) => Ok(contact),
            | None => Err(DomainError::ContactNotFound),
        }
    }
}
