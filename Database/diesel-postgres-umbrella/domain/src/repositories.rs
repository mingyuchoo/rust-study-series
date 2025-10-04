use super::entities::Todo;

pub trait TodoRepository {
    fn create(&mut self, title: &str) -> Todo;
    fn list(&mut self) -> Vec<Todo>;
    fn get(&mut self, id: i32) -> Option<Todo>;
    fn update(&mut self, id: i32, title: &str) -> Option<Todo>;
    fn delete(&mut self, id: i32) -> bool;
}
