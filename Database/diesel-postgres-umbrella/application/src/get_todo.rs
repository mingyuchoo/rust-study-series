use domain::{entities::Todo, repositories::TodoRepository};

pub struct GetTodoUseCase;

impl GetTodoUseCase {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(&self, repo: &mut dyn TodoRepository, id: i32) -> Option<Todo> {
        repo.get(id)
    }
}
