use crate::domain::services::repositories::entities::todo::{Todo, TodoForm};
use crate::domain::services::repositories::todo_repository::TodoRepository;

pub struct TodoService<R: TodoRepository> {
    repository: R,
}

impl<R: TodoRepository> TodoService<R> {
    pub fn new(repository: R) -> Self {
        Self {
            repository,
        }
    }

    pub async fn create(&self, todo_form: TodoForm) -> Result<Todo, Box<dyn std::error::Error>> { self.repository.create(todo_form).await }

    pub async fn update(&self, id: i32, todo_form: TodoForm) -> Result<Todo, Box<dyn std::error::Error>> { self.repository.update(id, todo_form).await }

    pub async fn delete(&self, id: i32) -> Result<(), Box<dyn std::error::Error>> { self.repository.delete(id).await }

    pub async fn find_by_id(&self, id: i32) -> Result<Todo, Box<dyn std::error::Error>> { self.repository.fetch_by_id(id).await }

    pub async fn find_all(&self) -> Result<Vec<Todo>, Box<dyn std::error::Error>> { self.repository.fetch_all().await }
}
