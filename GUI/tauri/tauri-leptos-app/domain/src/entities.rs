use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Contact {
    pub id: Uuid,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Contact {
    pub fn new(name: String, email: Option<String>, phone: Option<String>, address: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            email,
            phone,
            address,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update(&mut self, name: Option<String>, email: Option<String>, phone: Option<String>, address: Option<String>) {
        if let Some(name) = name {
            self.name = name;
        }
        if let Some(email) = email {
            self.email = Some(email);
        }
        if let Some(phone) = phone {
            self.phone = Some(phone);
        }
        if let Some(address) = address {
            self.address = Some(address);
        }
        self.updated_at = Utc::now();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateContactRequest {
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateContactRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
}
