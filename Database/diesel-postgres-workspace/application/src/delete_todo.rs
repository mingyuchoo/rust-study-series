use domain::repositories::TodoRepository;

pub struct DeleteTodoUseCase;

impl DeleteTodoUseCase {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(&self, repo: &mut dyn TodoRepository, id: i32) -> bool {
        repo.delete(id)
    }
}
