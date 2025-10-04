use axum::{
    routing::{get, post, put, delete},
    Router,
};
use super::{handlers, DbConn};

pub fn api_routes() -> Router<DbConn> {
    Router::new()
        .route("/todos", get(handlers::list_todos))
        .route("/todos", post(handlers::create_todo))
        .route("/todos/:id", get(handlers::get_todo))
        .route("/todos/:id", put(handlers::update_todo))
        .route("/todos/:id", delete(handlers::delete_todo))
}
