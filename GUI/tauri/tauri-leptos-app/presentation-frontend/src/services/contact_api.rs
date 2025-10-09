use domain::{Contact, ContactError, CreateContactRequest, UpdateContactRequest};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

pub struct ContactApi;

impl ContactApi {
    pub async fn create_contact(request: CreateContactRequest) -> Result<Contact, ContactError> {
        let args = serde_json::json!({ "request": request });
        let args = to_value(&args).unwrap();
        let result = invoke("create_contact", args).await;

        if let Ok(contact) = serde_wasm_bindgen::from_value::<Contact>(result.clone()) {
            Ok(contact)
        } else {
            let error_msg = serde_wasm_bindgen::from_value::<String>(result).unwrap_or_else(|_| "Unknown error".to_string());
            Err(ContactError::DatabaseError {
                message: error_msg,
            })
        }
    }

    pub async fn get_contact(id: &str) -> Result<Contact, ContactError> {
        let args = serde_json::json!({ "id": id });
        let args = to_value(&args).unwrap();
        let result = invoke("get_contact", args).await;

        if let Ok(contact) = serde_wasm_bindgen::from_value::<Contact>(result.clone()) {
            Ok(contact)
        } else {
            let error_msg = serde_wasm_bindgen::from_value::<String>(result).unwrap_or_else(|_| "Unknown error".to_string());
            Err(ContactError::DatabaseError {
                message: error_msg,
            })
        }
    }

    pub async fn get_all_contacts() -> Result<Vec<Contact>, ContactError> {
        let result = invoke("get_all_contacts", JsValue::NULL).await;

        if let Ok(contacts) = serde_wasm_bindgen::from_value::<Vec<Contact>>(result.clone()) {
            Ok(contacts)
        } else {
            let error_msg = serde_wasm_bindgen::from_value::<String>(result).unwrap_or_else(|_| "Unknown error".to_string());
            Err(ContactError::DatabaseError {
                message: error_msg,
            })
        }
    }

    pub async fn update_contact(id: &str, request: UpdateContactRequest) -> Result<Contact, ContactError> {
        let args = serde_json::json!({ "id": id, "request": request });
        let args = to_value(&args).unwrap();
        let result = invoke("update_contact", args).await;

        if let Ok(contact) = serde_wasm_bindgen::from_value::<Contact>(result.clone()) {
            Ok(contact)
        } else {
            let error_msg = serde_wasm_bindgen::from_value::<String>(result).unwrap_or_else(|_| "Unknown error".to_string());
            Err(ContactError::DatabaseError {
                message: error_msg,
            })
        }
    }

    pub async fn delete_contact(id: &str) -> Result<(), ContactError> {
        let args = serde_json::json!({ "id": id });
        let args = to_value(&args).unwrap();
        let result = invoke("delete_contact", args).await;

        if serde_wasm_bindgen::from_value::<()>(result.clone()).is_ok() {
            Ok(())
        } else {
            let error_msg = serde_wasm_bindgen::from_value::<String>(result).unwrap_or_else(|_| "Unknown error".to_string());
            Err(ContactError::DatabaseError {
                message: error_msg,
            })
        }
    }

    pub async fn search_contacts(query: &str) -> Result<Vec<Contact>, ContactError> {
        let args = serde_json::json!({ "query": query });
        let args = to_value(&args).unwrap();
        let result = invoke("search_contacts", args).await;

        if let Ok(contacts) = serde_wasm_bindgen::from_value::<Vec<Contact>>(result.clone()) {
            Ok(contacts)
        } else {
            let error_msg = serde_wasm_bindgen::from_value::<String>(result).unwrap_or_else(|_| "Unknown error".to_string());
            Err(ContactError::DatabaseError {
                message: error_msg,
            })
        }
    }
}
