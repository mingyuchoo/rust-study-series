use diesel::pg::PgConnection;
use crate::domain::repositories::TodoRepository;
use crate::domain::entities::Todo as DomainTodo;
use persistence::{self, PersistenceTodo};

fn map(p: PersistenceTodo) -> DomainTodo {
    DomainTodo { id: p.id, title: p.title }
}

impl TodoRepository for PgConnection {
    fn create(&mut self, title: &str) -> DomainTodo {
        let p = persistence::create_todo(self, title);
        map(p)
    }
    fn list(&mut self) -> Vec<DomainTodo> {
        persistence::list_todos(self).into_iter().map(map).collect()
    }
}
