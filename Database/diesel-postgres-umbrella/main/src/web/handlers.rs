use adapters::PgTodoRepository;
use application::{
    create_todo::CreateTodoUseCase,
    delete_todo::DeleteTodoUseCase,
    get_todo::GetTodoUseCase,
    list_todos::ListTodosUseCase,
    update_todo::UpdateTodoUseCase,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use domain::entities::Todo;
use serde::{Deserialize, Serialize};
use super::DbConn;

#[derive(Deserialize)]
pub struct CreateTodoRequest {
    title: String,
}

#[derive(Deserialize)]
pub struct UpdateTodoRequest {
    title: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
}

pub async fn list_todos(
    State(db_conn): State<DbConn>,
) -> Result<Json<Vec<Todo>>, StatusCode> {
    let mut conn = db_conn.lock().unwrap();
    let mut repo = PgTodoRepository::new(&mut conn);
    
    let use_case = ListTodosUseCase::new();
    let todos = use_case.execute(&mut repo);
    
    Ok(Json(todos))
}

pub async fn get_todo(
    State(db_conn): State<DbConn>,
    Path(id): Path<i32>,
) -> Result<Json<Todo>, StatusCode> {
    let mut conn = db_conn.lock().unwrap();
    let mut repo = PgTodoRepository::new(&mut conn);
    
    let use_case = GetTodoUseCase::new();
    match use_case.execute(&mut repo, id) {
        Some(todo) => Ok(Json(todo)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn create_todo(
    State(db_conn): State<DbConn>,
    Json(payload): Json<CreateTodoRequest>,
) -> Result<(StatusCode, Json<Todo>), StatusCode> {
    let mut conn = db_conn.lock().unwrap();
    let mut repo = PgTodoRepository::new(&mut conn);
    
    let use_case = CreateTodoUseCase::new();
    let todo = use_case.execute(&mut repo, &payload.title);
    
    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn update_todo(
    State(db_conn): State<DbConn>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTodoRequest>,
) -> Result<Json<Todo>, StatusCode> {
    let mut conn = db_conn.lock().unwrap();
    let mut repo = PgTodoRepository::new(&mut conn);
    
    let use_case = UpdateTodoUseCase::new();
    match use_case.execute(&mut repo, id, &payload.title) {
        Some(todo) => Ok(Json(todo)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn delete_todo(
    State(db_conn): State<DbConn>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let mut conn = db_conn.lock().unwrap();
    let mut repo = PgTodoRepository::new(&mut conn);
    
    let use_case = DeleteTodoUseCase::new();
    if use_case.execute(&mut repo, id) {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
