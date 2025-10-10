use crate::models::*;
use application::*;
use std::sync::Arc;
use tauri::State;
use uuid::Uuid;

pub struct AppState {
    pub create_contact_use_case: Arc<CreateContactUseCase>,
    pub get_contact_use_case: Arc<GetContactUseCase>,
    pub list_contacts_use_case: Arc<ListContactsUseCase>,
    pub update_contact_use_case: Arc<UpdateContactUseCase>,
    pub delete_contact_use_case: Arc<DeleteContactUseCase>,
    pub search_contacts_use_case: Arc<SearchContactsUseCase>,
}

#[tauri::command]
pub async fn create_contact(state: State<'_, AppState>, request: CreateContactRequest) -> Result<ContactDto, String> {
    state
        .create_contact_use_case
        .execute(request.name, request.email, request.phone, request.address)
        .await
        .map(ContactDto::from)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_contact(state: State<'_, AppState>, id: String) -> Result<ContactDto, String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;

    state.get_contact_use_case.execute(uuid).await.map(ContactDto::from).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_contacts(state: State<'_, AppState>) -> Result<Vec<ContactDto>, String> {
    state
        .list_contacts_use_case
        .execute()
        .await
        .map(|contacts| contacts.into_iter().map(ContactDto::from).collect())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_contact(state: State<'_, AppState>, request: UpdateContactRequest) -> Result<ContactDto, String> {
    let uuid = Uuid::parse_str(&request.id).map_err(|e| e.to_string())?;

    state
        .update_contact_use_case
        .execute(uuid, request.name, request.email, request.phone, request.address)
        .await
        .map(ContactDto::from)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_contact(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;

    state.delete_contact_use_case.execute(uuid).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn search_contacts(state: State<'_, AppState>, query: String) -> Result<Vec<ContactDto>, String> {
    state
        .search_contacts_use_case
        .execute(&query)
        .await
        .map(|contacts| contacts.into_iter().map(ContactDto::from).collect())
        .map_err(|e| e.to_string())
}
