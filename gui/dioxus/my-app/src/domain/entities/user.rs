use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub username: String,
    pub email: String,
    pub address: Option<Address>,
    pub phone: Option<String>,
    pub website: Option<String>,
    pub company: Option<Company>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Address {
    pub street: String,
    pub suite: String,
    pub city: String,
    pub zipcode: String,
    pub geo: Geo,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Geo {
    pub lat: String,
    pub lng: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[allow(non_snake_case)]
pub struct Company {
    pub name: String,
    pub catchPhrase: String,
    pub bs: String,
}

// New user form data
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserForm {
    pub name: String,
    pub username: String,
    pub email: String,
    pub phone: String,
}
