mod handlers;
mod routes;

use axum::Router;
use diesel::pg::PgConnection;
use std::sync::{Arc, Mutex};
use tower_http::services::ServeDir;

pub type DbConn = Arc<Mutex<PgConnection>>;

pub async fn start_server(db_conn: DbConn) {
    let app = Router::new()
        .nest("/api", routes::api_routes())
        .fallback_service(ServeDir::new("main/static"))
        .with_state(db_conn);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    axum::serve(listener, app).await.unwrap();
}
