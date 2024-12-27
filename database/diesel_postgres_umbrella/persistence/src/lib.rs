mod models;
mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use models::{NewTodo, Todo};
use std::env;

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
