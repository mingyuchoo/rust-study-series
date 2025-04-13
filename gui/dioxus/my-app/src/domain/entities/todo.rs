use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[allow(non_snake_case)]
pub struct Todo {
    pub id: i32,
    pub userId: i32,
    pub title: String,
    pub completed: bool,
}

// New todo form data
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[allow(non_snake_case)]
pub struct TodoForm {
    pub userId: i32,
    pub title: String,
    pub completed: bool,
}
