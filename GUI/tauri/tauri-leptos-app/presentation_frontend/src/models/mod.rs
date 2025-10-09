use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAddressRequest {
    pub name: String,
    pub phone: String,
    pub email: String,
    pub street: String,
    pub city: String,
    pub postal_code: String,
    pub country: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAddressRequest {
    pub id: Uuid,
    pub name: String,
    pub phone: String,
    pub email: String,
    pub street: String,
    pub city: String,
    pub postal_code: String,
    pub country: String,
}