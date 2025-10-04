mod web;

use infra::db::DbProvider;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    // Initialize database connection and run migrations
    let mut conn = DbProvider::establish();
    DbProvider::migrate_and_seed(&mut conn);
    
    // Wrap connection in Arc<Mutex> for thread-safe sharing
    let db_conn = Arc::new(Mutex::new(conn));
    
    println!("🚀 Starting Todo Web Service...");
    println!("📝 API available at: http://localhost:3000/api/todos");
    println!("🌐 Web UI available at: http://localhost:3000");
    
    web::start_server(db_conn).await;
}
