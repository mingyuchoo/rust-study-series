use diesel::pg::PgConnection;
use domain::repositories::TodoRepository;
use domain::entities::Todo as DomainTodo;
use infra::{self, InfraTodo};

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
        let p = infra::create_todo(self.conn, title);
        map(p)
    }
    fn list(&mut self) -> Vec<DomainTodo> {
        infra::list_todos(self.conn).into_iter().map(map).collect()
    }
}
