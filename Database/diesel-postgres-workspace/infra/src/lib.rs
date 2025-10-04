mod models;
mod schema;
pub mod db;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use models::{NewTodo, Todo};
use std::env;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub use models::{NewTodo as InfraNewTodo, Todo as InfraTodo};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url: String =
        env::var("DATABASE_URL").expect("DATABASE must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connection to {database_url}"))
}

pub fn create_todo(conn: &mut PgConnection, title: &str) -> Todo {
    use schema::todo;

    let new_todo = NewTodo { title };

    diesel::insert_into(todo::table)
        .values(&new_todo)
        .returning(Todo::as_returning())
        .get_result(conn)
        .expect("Error saving new todo")
}

pub fn list_todos(conn: &mut PgConnection) -> Vec<Todo> {
    use schema::todo::dsl::*;
    todo
        .select(Todo::as_select())
        .load::<Todo>(conn)
        .expect("Error loading todos")
}

pub fn get_todo(conn: &mut PgConnection, todo_id: i32) -> Option<Todo> {
    use schema::todo::dsl::*;
    todo
        .find(todo_id)
        .select(Todo::as_select())
        .first(conn)
        .ok()
}

pub fn update_todo(conn: &mut PgConnection, todo_id: i32, new_title: &str) -> Option<Todo> {
    use schema::todo::dsl::*;
    diesel::update(todo.find(todo_id))
        .set(title.eq(new_title))
        .returning(Todo::as_returning())
        .get_result(conn)
        .ok()
}

pub fn delete_todo(conn: &mut PgConnection, todo_id: i32) -> bool {
    use schema::todo::dsl::*;
    diesel::delete(todo.find(todo_id))
        .execute(conn)
        .map(|rows| rows > 0)
        .unwrap_or(false)
}

pub fn run_migrations_and_seed(conn: &mut PgConnection) {
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");

    // Seed only if empty
    use schema::todo::dsl::*;
    let count: i64 = todo
        .count()
        .get_result(conn)
        .expect("Failed to count todos");

    if count == 0 {
        let samples = [
            "Learn Diesel",
            "Model domain entities",
            "Implement use cases",
        ];
        for t in samples {
            let _ = create_todo(conn, t);
        }
    }
}
