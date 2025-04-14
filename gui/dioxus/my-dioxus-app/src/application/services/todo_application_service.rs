use crate::domain::services::repositories::entities::todo::{Todo, TodoForm};
use crate::domain::services::repositories::todo_repository::TodoRepository;
use crate::domain::services::todo_service::TodoService;

pub struct TodoApplicationService<R: TodoRepository> {
    todo_service: TodoService<R>,
}

impl<R: TodoRepository> TodoApplicationService<R> {
    pub fn new(todo_service: TodoService<R>) -> Self {
        Self {
            todo_service,
        }
    }

    pub async fn create(&self, todo_form: TodoForm) -> Result<Todo, Box<dyn std::error::Error>> { self.todo_service.create(todo_form).await }

    pub async fn update(&self, id: i32, todo_form: TodoForm) -> Result<Todo, Box<dyn std::error::Error>> { self.todo_service.update(id, todo_form).await }

    pub async fn delete(&self, id: i32) -> Result<(), Box<dyn std::error::Error>> { self.todo_service.delete(id).await }

    pub async fn find_by_id(&self, id: i32) -> Result<Todo, Box<dyn std::error::Error>> { self.todo_service.find_by_id(id).await }

    pub async fn find_all(&self) -> Result<Vec<Todo>, Box<dyn std::error::Error>> { self.todo_service.find_all().await }
}

#[derive(Clone, Debug)]
pub struct TodoDto {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}
impl From<Todo> for TodoDto {
    fn from(todo: Todo) -> Self {
        Self {
            id: todo.id,
            title: todo.title,
            completed: todo.completed,
        }
    }
}
