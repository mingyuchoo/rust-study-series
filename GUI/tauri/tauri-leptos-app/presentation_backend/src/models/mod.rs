use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressResponse {
    pub id: Uuid,
    pub name: String,
    pub phone: String,
    pub email: String,
    pub street: String,
    pub city: String,
    pub postal_code: String,
    pub country: String,
}

impl From<domain::entities::Address> for AddressResponse {
    fn from(address: domain::entities::Address) -> Self {
        Self {
            id: address.id,
            name: address.name,
            phone: address.phone,
            email: address.email,
            street: address.street,
            city: address.city,
            postal_code: address.postal_code,
            country: address.country,
        }
    }
}