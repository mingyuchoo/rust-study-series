use crate::models::todo::Todo;
use leptos::*;

#[server]
pub async fn fetch_todos() -> Result<Vec<Todo>, ServerFnError> {
    use std::fs;
    let todos_json = fs::read_to_string("assets/todos.json")?;
    let todos: Vec<Todo> = serde_json::from_str(&todos_json)?;
    Ok(todos)
}
