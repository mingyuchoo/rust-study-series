use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[allow(non_snake_case)]
pub struct Post {
    pub id: i32,
    pub userId: i32,
    pub title: String,
    pub body: String,
}

// New post form data
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[allow(non_snake_case)]
pub struct PostForm {
    pub userId: i32,
    pub title: String,
    pub body: String,
}
