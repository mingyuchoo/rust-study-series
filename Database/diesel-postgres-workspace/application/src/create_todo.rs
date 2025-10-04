use domain::repositories::TodoRepository;
use domain::entities::Todo;

pub struct CreateTodoUseCase;

impl CreateTodoUseCase {
    pub fn new() -> Self { Self }
    pub fn execute<R: TodoRepository>(&self, repo: &mut R, title: &str) -> Todo {
        repo.create(title)
    }
}
