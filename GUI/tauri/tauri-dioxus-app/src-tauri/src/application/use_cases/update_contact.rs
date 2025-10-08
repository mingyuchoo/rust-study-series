use crate::domain::{Contact, ContactRepository, DomainError};
use std::sync::Arc;
use uuid::Uuid;

pub struct UpdateContactUseCase {
    repository: Arc<dyn ContactRepository>,
}

impl UpdateContactUseCase {
    pub fn new(repository: Arc<dyn ContactRepository>) -> Self {
        Self {
            repository,
        }
    }

    pub async fn execute(
        &self,
        id: Uuid,
        name: Option<String>,
        email: Option<String>,
        phone: Option<String>,
        address: Option<String>,
    ) -> Result<Contact, DomainError> {
        let mut contact = match self.repository.get_by_id(id).await? {
            | Some(contact) => contact,
            | None => return Err(DomainError::ContactNotFound),
        };

        if let Some(ref name) = name {
            if name.trim().is_empty() {
                return Err(DomainError::InvalidContactData("Name cannot be empty".to_string()));
            }
        }

        contact.update(name, email, phone, address);
        self.repository.update(contact).await
    }
}
