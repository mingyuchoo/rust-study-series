use application::ContactService;
use domain::{Contact, CreateContactRequest, UpdateContactRequest};
use infrastructure::SqliteContactRepository;
use std::sync::Arc;

#[tauri::command]
async fn create_contact(request: CreateContactRequest, state: tauri::State<'_, ContactService>) -> Result<Contact, String> {
    state.create_contact(request).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_contact(id: String, state: tauri::State<'_, ContactService>) -> Result<Contact, String> {
    let uuid = uuid::Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    state.get_contact(uuid).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_all_contacts(state: tauri::State<'_, ContactService>) -> Result<Vec<Contact>, String> { state.get_all_contacts().await.map_err(|e| e.to_string()) }

#[tauri::command]
async fn update_contact(id: String, request: UpdateContactRequest, state: tauri::State<'_, ContactService>) -> Result<Contact, String> {
    let uuid = uuid::Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    state.update_contact(uuid, request).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_contact(id: String, state: tauri::State<'_, ContactService>) -> Result<(), String> {
    let uuid = uuid::Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    state.delete_contact(uuid).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn search_contacts(query: String, state: tauri::State<'_, ContactService>) -> Result<Vec<Contact>, String> {
    state.search_contacts(&query).await.map_err(|e| e.to_string())
}

#[tokio::main]
async fn main() {
    // Initialize database in memory for now
    let database_url = "sqlite::memory:";
    let repository = Arc::new(SqliteContactRepository::new(database_url).await.expect("Failed to initialize database"));
    let contact_service = ContactService::new(repository);

    tauri::Builder::default()
        .manage(contact_service)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            create_contact,
            get_contact,
            get_all_contacts,
            update_contact,
            delete_contact,
            search_contacts
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
