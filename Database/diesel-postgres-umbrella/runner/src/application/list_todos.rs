use crate::domain::repositories::TodoRepository;
use crate::domain::entities::Todo;

pub struct ListTodosUseCase;

impl ListTodosUseCase {
    pub fn new() -> Self { Self }
    pub fn execute<R: TodoRepository>(&self, repo: &mut R) -> Vec<Todo> {
        repo.list()
    }
}
