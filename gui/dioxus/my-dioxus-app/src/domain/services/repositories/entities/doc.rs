use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Doc {
    pub id: i32,
    pub title: String,
    pub contents: String,
    pub archived: bool,
}

// New user form data
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocForm {
    pub title: String,
    pub contents: String,
    pub archived: bool,
}
