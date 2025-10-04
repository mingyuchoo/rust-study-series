use domain::{entities::Todo, repositories::TodoRepository};

pub struct UpdateTodoUseCase;

impl UpdateTodoUseCase {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(&self, repo: &mut dyn TodoRepository, id: i32, title: &str) -> Option<Todo> {
        repo.update(id, title)
    }
}
