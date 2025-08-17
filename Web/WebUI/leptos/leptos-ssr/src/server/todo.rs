use crate::models::todo::Todo;
use leptos::*;
use std::fs;

const TODOS_FILE_PATH: &str = "assets/todos.json";

#[derive(Debug, thiserror::Error)]
pub enum TodoError {
    #[error("Failed to read todos file: {0}")]
    FileError(#[from] std::io::Error),

    #[error("Failed to parse todos json: {0}")]
    ParseError(#[from] serde_json::Error),
}

#[server]
pub async fn fetch_todos() -> Result<Vec<Todo>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        load_todos_from_file().map_err(ServerFnError::from)
    }
    #[cfg(all(not(feature = "ssr"), feature = "csr"))]
    {
        Err(ServerFnError::ServerError("Server-side code running on client".into()))
    }
}

fn load_todos_from_file() -> Result<Vec<Todo>, TodoError> {
    let todos_json = fs::read_to_string(TODOS_FILE_PATH)?;
    let todos = serde_json::from_str(&todos_json)?;
    Ok(todos)
}
