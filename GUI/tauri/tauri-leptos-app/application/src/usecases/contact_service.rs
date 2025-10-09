use domain::{Contact, ContactError, ContactRepository, ContactResult, CreateContactRequest, UpdateContactRequest};
use std::sync::Arc;
use uuid::Uuid;

pub struct ContactService {
    repository: Arc<dyn ContactRepository>,
}

impl ContactService {
    pub fn new(repository: Arc<dyn ContactRepository>) -> Self {
        Self {
            repository,
        }
    }

    pub async fn create_contact(&self, request: CreateContactRequest) -> ContactResult<Contact> {
        if request.name.trim().is_empty() {
            return Err(ContactError::EmptyName);
        }

        if let Some(ref email) = request.email {
            if !email.trim().is_empty() && !self.is_valid_email(email) {
                return Err(ContactError::InvalidEmail {
                    email: email.clone(),
                });
            }
        }

        let contact = Contact::new(
            request.name.trim().to_string(),
            request.email.map(|e| e.trim().to_string()).filter(|e| !e.is_empty()),
            request.phone.map(|p| p.trim().to_string()).filter(|p| !p.is_empty()),
            request.address.map(|a| a.trim().to_string()).filter(|a| !a.is_empty()),
        );

        self.repository.create(contact).await
    }

    pub async fn get_contact(&self, id: Uuid) -> ContactResult<Contact> { self.repository.get_by_id(id).await }

    pub async fn get_all_contacts(&self) -> ContactResult<Vec<Contact>> { self.repository.get_all().await }

    pub async fn update_contact(&self, id: Uuid, request: UpdateContactRequest) -> ContactResult<Contact> {
        let mut contact = self.repository.get_by_id(id).await?;

        if let Some(ref name) = request.name {
            if name.trim().is_empty() {
                return Err(ContactError::EmptyName);
            }
        }

        if let Some(ref email) = request.email {
            if !email.trim().is_empty() && !self.is_valid_email(email) {
                return Err(ContactError::InvalidEmail {
                    email: email.clone(),
                });
            }
        }

        contact.update(
            request.name.map(|n| n.trim().to_string()),
            request.email.map(|e| e.trim().to_string()).filter(|e| !e.is_empty()),
            request.phone.map(|p| p.trim().to_string()).filter(|p| !p.is_empty()),
            request.address.map(|a| a.trim().to_string()).filter(|a| !a.is_empty()),
        );

        self.repository.update(contact).await
    }

    pub async fn delete_contact(&self, id: Uuid) -> ContactResult<()> { self.repository.delete(id).await }

    pub async fn search_contacts(&self, query: &str) -> ContactResult<Vec<Contact>> {
        if query.trim().is_empty() {
            return self.get_all_contacts().await;
        }
        self.repository.search(query.trim()).await
    }

    fn is_valid_email(&self, email: &str) -> bool { email.contains('@') && email.contains('.') && email.len() > 5 }
}
