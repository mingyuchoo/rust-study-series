use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Todo {
    pub user_id: i32,
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

