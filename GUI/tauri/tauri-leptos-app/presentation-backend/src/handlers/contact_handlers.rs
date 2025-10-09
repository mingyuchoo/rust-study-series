use application::ContactService;
use domain::{Contact, CreateContactRequest, UpdateContactRequest};

#[tauri::command]
pub async fn create_contact(request: CreateContactRequest, state: tauri::State<'_, ContactService>) -> Result<Contact, String> {
    state.create_contact(request).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_contact(id: String, state: tauri::State<'_, ContactService>) -> Result<Contact, String> {
    let uuid = uuid::Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    state.get_contact(uuid).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_all_contacts(state: tauri::State<'_, ContactService>) -> Result<Vec<Contact>, String> {
    state.get_all_contacts().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_contact(id: String, request: UpdateContactRequest, state: tauri::State<'_, ContactService>) -> Result<Contact, String> {
    let uuid = uuid::Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    state.update_contact(uuid, request).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_contact(id: String, state: tauri::State<'_, ContactService>) -> Result<(), String> {
    let uuid = uuid::Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    state.delete_contact(uuid).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn search_contacts(query: String, state: tauri::State<'_, ContactService>) -> Result<Vec<Contact>, String> {
    state.search_contacts(&query).await.map_err(|e| e.to_string())
}
