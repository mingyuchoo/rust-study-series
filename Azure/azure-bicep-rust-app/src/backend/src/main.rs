use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, put, delete, web, middleware::Logger, Result};
use actix_files as fs;
use clap::Parser;
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, Row};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: String,
    title: String,
    description: Option<String>,
    completed: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
struct CreateTodo {
    title: String,
    description: Option<String>,
}

#[derive(Deserialize)]
struct UpdateTodo {
    title: Option<String>,
    description: Option<String>,
    completed: Option<bool>,
}

async fn init_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS todos (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT,
            completed BOOLEAN NOT NULL DEFAULT FALSE,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;
    
    Ok(())
}

#[get("/todos")]
async fn get_todos(pool: web::Data<SqlitePool>) -> Result<impl Responder> {
    let rows = sqlx::query("SELECT * FROM todos ORDER BY created_at DESC")
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    let todos: Vec<Todo> = rows
        .iter()
        .map(|row| Todo {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            completed: row.get("completed"),
            created_at: row.get::<String, _>("created_at").parse().unwrap_or_else(|_| Utc::now()),
            updated_at: row.get::<String, _>("updated_at").parse().unwrap_or_else(|_| Utc::now()),
        })
        .collect();

    Ok(HttpResponse::Ok().json(todos))
}

#[post("/todos")]
async fn create_todo(
    pool: web::Data<SqlitePool>,
    todo: web::Json<CreateTodo>,
) -> Result<impl Responder> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now();
    
    sqlx::query(
        "INSERT INTO todos (id, title, description, completed, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&todo.title)
    .bind(&todo.description)
    .bind(false)
    .bind(now.to_rfc3339())
    .bind(now.to_rfc3339())
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    let new_todo = Todo {
        id,
        title: todo.title.clone(),
        description: todo.description.clone(),
        completed: false,
        created_at: now,
        updated_at: now,
    };

    Ok(HttpResponse::Created().json(new_todo))
}

#[put("/todos/{id}")]
async fn update_todo(
    pool: web::Data<SqlitePool>,
    path: web::Path<String>,
    todo: web::Json<UpdateTodo>,
) -> Result<impl Responder> {
    let id = path.into_inner();
    let now = Utc::now();

    // First check if todo exists
    let existing = sqlx::query("SELECT * FROM todos WHERE id = ?")
        .bind(&id)
        .fetch_optional(pool.get_ref())
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    if existing.is_none() {
        return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Todo not found"
        })));
    }

    let existing_row = existing.unwrap();
    
    // Get existing values
    let existing_title: String = existing_row.get("title");
    let existing_description: Option<String> = existing_row.get("description");
    let existing_completed: bool = existing_row.get("completed");
    
    // Update with new values or keep existing ones
    let title = todo.title.as_ref().unwrap_or(&existing_title);
    let description = todo.description.as_ref().or(existing_description.as_ref());
    let completed = todo.completed.unwrap_or(existing_completed);

    sqlx::query(
        "UPDATE todos SET title = ?, description = ?, completed = ?, updated_at = ? WHERE id = ?"
    )
    .bind(title)
    .bind(description)
    .bind(completed)
    .bind(now.to_rfc3339())
    .bind(&id)
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    let created_at_str: String = existing_row.get("created_at");
    let updated_todo = Todo {
        id,
        title: title.clone(),
        description: description.cloned(),
        completed,
        created_at: created_at_str.parse().unwrap_or_else(|_| Utc::now()),
        updated_at: now,
    };

    Ok(HttpResponse::Ok().json(updated_todo))
}

#[delete("/todos/{id}")]
async fn delete_todo(
    pool: web::Data<SqlitePool>,
    path: web::Path<String>,
) -> Result<impl Responder> {
    let id = path.into_inner();

    let result = sqlx::query("DELETE FROM todos WHERE id = ?")
        .bind(&id)
        .execute(pool.get_ref())
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    if result.rows_affected() == 0 {
        return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Todo not found"
        })));
    }

    Ok(HttpResponse::NoContent().finish())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    // Initialize SQLite database
    let current_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let db_path = current_dir.join("todos.db");
    let database_url = format!("sqlite:{}?mode=rwc", db_path.display());
    println!("Current directory: {:?}", current_dir);
    println!("Connecting to database: {}", database_url);
    
    let pool = match SqlitePool::connect(&database_url).await {
        Ok(pool) => {
            println!("Successfully connected to database");
            pool
        }
        Err(e) => {
            eprintln!("Failed to connect to database: {}", e);
            eprintln!("Make sure the current directory is writable");
            std::process::exit(1);
        }
    };

    // Initialize database schema
    init_db(&pool)
        .await
        .expect("Failed to initialize database");

    println!("Starting TODO server on port {}", args.port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            // API routes
            .service(
                web::scope("/api")
                    .service(get_todos)
                    .service(create_todo)
                    .service(update_todo)
                    .service(delete_todo)
            )
            // Static files (frontend)
            .service(fs::Files::new("/", "./wwwroot").index_file("index.html"))
    })
    .bind(("0.0.0.0", args.port))?
    .run()
    .await
}
