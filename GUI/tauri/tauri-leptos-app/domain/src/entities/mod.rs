use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Address {
    pub id: Uuid,
    pub name: String,
    pub phone: String,
    pub email: String,
    pub street: String,
    pub city: String,
    pub postal_code: String,
    pub country: String,
}

impl Address {
    pub fn new(
        name: String,
        phone: String,
        email: String,
        street: String,
        city: String,
        postal_code: String,
        country: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            phone,
            email,
            street,
            city,
            postal_code,
            country,
        }
    }
}