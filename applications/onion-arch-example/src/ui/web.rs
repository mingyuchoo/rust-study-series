// src/ui/web.rs - Web server for User CRUD UI
//

use crate::infrastructure::db::user_db_controller::UserApiController;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

// Shared controller type
pub type SharedController = Arc<Mutex<UserApiController<crate::infrastructure::db::repositories::user_db_repository::UserDbRepository>>>;

#[derive(Serialize, Deserialize)]
pub struct UserInput {
    pub id: String,
    pub username: String,
    pub email: String,
}

async fn list_users(State(controller): State<SharedController>) -> Response {
    let ctrl = controller.lock().await;
    match ctrl.list_all_users_json() {
        | Ok(users) => axum::Json::<Vec<crate::application::services::user_application_service::UserDto>>(users).into_response(),
        | Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn get_user(Path(id): Path<String>, State(controller): State<SharedController>) -> Response {
    let ctrl = controller.lock().await;
    match ctrl.get_user(&id) {
        | Ok(user) => Html(format!("<pre>{}</pre>", user)).into_response(),
        | Err(e) => (StatusCode::NOT_FOUND, e.to_string()).into_response(),
    }
}

async fn create_user(State(controller): State<SharedController>, Json(user): Json<UserInput>) -> Response {
    let ctrl = controller.lock().await;
    match ctrl.register_user(user.id, user.username, user.email) {
        | Ok(msg) => Html(format!("<pre>{}</pre>", msg)).into_response(),
        | Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

async fn update_user(Path(id): Path<String>, State(controller): State<SharedController>) -> Response {
    let ctrl = controller.lock().await;
    match ctrl.deactivate_user(&id) {
        | Ok(msg) => Html(format!("<pre>{}</pre>", msg)).into_response(),
        | Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

async fn delete_user(Path(id): Path<String>, State(controller): State<SharedController>) -> Response {
    let ctrl = controller.lock().await;
    match ctrl.delete_user(&id) {
        | Ok(msg) => Html(format!("<pre>{}</pre>", msg)).into_response(),
        | Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

async fn index() -> Html<&'static str> { Html(include_str!("./static/index.html")) }

pub async fn main() {
    let db_path = "users.db";
    let controller = UserApiController::new_with_db_path(db_path).expect("Failed to init controller");
    let shared = Arc::new(Mutex::new(controller));

    // Create the router with the shared state
    let app = Router::new()
        .route("/", get(index))
        .route("/api/users", get(list_users).post(create_user))
        .route("/api/users/:id", get(get_user).put(update_user).delete(delete_user))
        .with_state(shared);

    println!("Web UI running at http://localhost:3000");

    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
