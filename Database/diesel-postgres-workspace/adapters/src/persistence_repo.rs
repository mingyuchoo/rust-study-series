use diesel::pg::PgConnection;
use domain::repositories::TodoRepository;
use domain::entities::Todo as DomainTodo;
use infrastructure::{self, InfraTodo};

fn map(p: InfraTodo) -> DomainTodo {
    DomainTodo { id: p.id, title: p.title }
}

pub struct PgTodoRepository<'a> {
    conn: &'a mut PgConnection,
}

impl<'a> PgTodoRepository<'a> {
    pub fn new(conn: &'a mut PgConnection) -> Self {
        Self { conn }
    }
}

impl<'a> TodoRepository for PgTodoRepository<'a> {
    fn create(&mut self, title: &str) -> DomainTodo {
        let p = infrastructure::create_todo(self.conn, title);
        map(p)
    }
    fn list(&mut self) -> Vec<DomainTodo> {
        infrastructure::list_todos(self.conn).into_iter().map(map).collect()
    }
    fn get(&mut self, id: i32) -> Option<DomainTodo> {
        infrastructure::get_todo(self.conn, id).map(map)
    }
    fn update(&mut self, id: i32, title: &str) -> Option<DomainTodo> {
        infrastructure::update_todo(self.conn, id, title).map(map)
    }
    fn delete(&mut self, id: i32) -> bool {
        infrastructure::delete_todo(self.conn, id)
    }
}
