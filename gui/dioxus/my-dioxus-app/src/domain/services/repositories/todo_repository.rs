use crate::domain::services::repositories::entities::todo::{Todo, TodoForm};
use std::error::Error;

#[async_trait::async_trait(?Send)]
pub trait TodoRepository {
    async fn fetch_all(&self) -> Result<Vec<Todo>, Box<dyn Error>>;
    #[allow(dead_code)]
    async fn fetch_by_id(&self, id: i32) -> Result<Todo, Box<dyn Error>>;
    async fn create(&self, todo: TodoForm) -> Result<Todo, Box<dyn Error>>;
    async fn update(&self, id: i32, todo: TodoForm) -> Result<Todo, Box<dyn Error>>;
    async fn delete(&self, id: i32) -> Result<(), Box<dyn Error>>;
}
