use crate::domain::{Contact, ContactRepository, DomainError};
use std::sync::Arc;

pub struct CreateContactUseCase {
    repository: Arc<dyn ContactRepository>,
}

impl CreateContactUseCase {
    pub fn new(repository: Arc<dyn ContactRepository>) -> Self {
        Self {
            repository,
        }
    }

    pub async fn execute(&self, name: String, email: Option<String>, phone: Option<String>, address: Option<String>) -> Result<Contact, DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::InvalidContactData("Name cannot be empty".to_string()));
        }

        let contact = Contact::new(name, email, phone, address);
        self.repository.create(contact).await
    }
}
