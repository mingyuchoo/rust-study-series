use crate::{models::*, AppState};
use domain::entities::Address;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub async fn create_address(
    request: CreateAddressRequest,
    state: State<'_, AppState>,
) -> Result<AddressResponse, String> {
    let address = state
        .create_address(
            request.name,
            request.phone,
            request.email,
            request.street,
            request.city,
            request.postal_code,
            request.country,
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(AddressResponse::from(address))
}

#[tauri::command]
pub async fn get_address(
    id: String,
    state: State<'_, AppState>,
) -> Result<Option<AddressResponse>, String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let address = state
        .get_address(uuid)
        .await
        .map_err(|e| e.to_string())?;

    Ok(address.map(AddressResponse::from))
}

#[tauri::command]
pub async fn get_all_addresses(
    state: State<'_, AppState>,
) -> Result<Vec<AddressResponse>, String> {
    let addresses = state
        .get_all_addresses()
        .await
        .map_err(|e| e.to_string())?;

    Ok(addresses.into_iter().map(AddressResponse::from).collect())
}

#[tauri::command]
pub async fn update_address(
    request: UpdateAddressRequest,
    state: State<'_, AppState>,
) -> Result<AddressResponse, String> {
    let address = Address {
        id: request.id,
        name: request.name,
        phone: request.phone,
        email: request.email,
        street: request.street,
        city: request.city,
        postal_code: request.postal_code,
        country: request.country,
    };

    let updated_address = state
        .update_address(address)
        .await
        .map_err(|e| e.to_string())?;

    Ok(AddressResponse::from(updated_address))
}

#[tauri::command]
pub async fn delete_address(
    id: String,
    state: State<'_, AppState>,
) -> Result<bool, String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    state
        .delete_address(uuid)
        .await
        .map_err(|e| e.to_string())
}