pub mod models;
pub mod commands;

use application::usecases::AddressService;
use infrastructure::database::SqliteAddressRepository;
use sqlx::SqlitePool;
use std::sync::Arc;

pub type AppState = Arc<AddressService>;

pub async fn create_app_state() -> Result<AppState, Box<dyn std::error::Error>> {
    // Use in-memory database for development
    let db_url = "sqlite::memory:";
    
    println!("Database URL: {}", db_url);
    
    let pool = SqlitePool::connect(db_url).await?;
    let repository = SqliteAddressRepository::new(pool);
    repository.init_database().await?;
    
    let service = AddressService::new(Box::new(repository));
    Ok(Arc::new(service))
}

pub fn setup_tauri_app(builder: tauri::Builder<tauri::Wry>) -> tauri::Builder<tauri::Wry> {
    builder.invoke_handler(tauri::generate_handler![
        commands::create_address,
        commands::get_address,
        commands::get_all_addresses,
        commands::update_address,
        commands::delete_address
    ])
}