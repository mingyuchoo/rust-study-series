use super::entities::Todo;

pub trait TodoRepository {
    fn create(&mut self, title: &str) -> Todo;
    fn list(&mut self) -> Vec<Todo>;
}
