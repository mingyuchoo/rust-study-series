use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::models::Address;
use uuid::Uuid;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn AddressItem<F>(
    address: Address,
    on_delete: F,
) -> impl IntoView 
where
    F: Fn(Uuid) + 'static + Copy,
{
    let (is_deleting, set_is_deleting) = signal(false);

    let delete_address = move |_| {
        if is_deleting.get() {
            return;
        }

        set_is_deleting.set(true);
        let id = address.id;
        
        spawn_local(async move {
            // Create a wrapper struct for the Tauri command arguments
            #[derive(serde::Serialize)]
            struct DeleteAddressArgs {
                id: String,
            }
            
            let args = DeleteAddressArgs { id: id.to_string() };
            let args = serde_wasm_bindgen::to_value(&args).unwrap();
            let result = invoke("delete_address", args).await;
            
            // Try direct deserialization first (Tauri unwraps Result automatically)
            match serde_wasm_bindgen::from_value::<bool>(result.clone()) {
                Ok(true) => {
                    // Reset before triggering deletion callback to avoid setting a disposed signal
                    set_is_deleting.set(false);
                    on_delete(id);
                    return;
                }
                Ok(false) => {
                    web_sys::console::error_1(&"Failed to delete address".into());
                    set_is_deleting.set(false);
                    return;
                }
                Err(err) => {
                    web_sys::console::log_1(&format!("Direct bool deserialization failed: {:?}", err).into());
                }
            }
            
            // Fallback to Result wrapper
            match serde_wasm_bindgen::from_value::<Result<bool, String>>(result) {
                Ok(Ok(true)) => {
                    // Reset before triggering deletion callback to avoid setting a disposed signal
                    set_is_deleting.set(false);
                    on_delete(id);
                    return;
                }
                Ok(Ok(false)) => {
                    web_sys::console::error_1(&"Failed to delete address".into());
                }
                Ok(Err(err)) => {
                    web_sys::console::error_1(&format!("Error: {}", err).into());
                }
                Err(err) => {
                    web_sys::console::error_1(&format!("Parse error: {:?}", err).into());
                }
            }
            set_is_deleting.set(false);
        });
    };

    view! {
        <div class="address-item">
            <div class="address-info">
                <h3>{address.name.clone()}</h3>
                <p><strong>"전화:"</strong> {address.phone.clone()}</p>
                <p><strong>"이메일:"</strong> {address.email.clone()}</p>
            </div>
            <div class="address-actions">
                <button 
                    class="btn btn-danger"
                    on:click=delete_address
                    disabled=is_deleting
                >
                    {move || if is_deleting.get() { "삭제 중..." } else { "삭제" }}
                </button>
            </div>
        </div>
    }
}