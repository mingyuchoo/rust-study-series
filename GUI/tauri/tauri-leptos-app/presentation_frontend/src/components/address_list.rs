use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::models::Address;
use crate::components::address_item::AddressItem;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn AddressList(refresh_trigger: ReadSignal<i32>) -> impl IntoView {
    let (addresses, set_addresses) = signal(Vec::<Address>::new());
    let (loading, set_loading) = signal(true);

    let load_addresses = move || {
        set_loading.set(true);
        spawn_local(async move {
            let result = invoke("get_all_addresses", JsValue::NULL).await;
            match serde_wasm_bindgen::from_value::<Result<Vec<Address>, String>>(result) {
                Ok(Ok(addr_list)) => {
                    set_addresses.set(addr_list);
                }
                Ok(Err(err)) => {
                    web_sys::console::error_1(&format!("Error loading addresses: {}", err).into());
                }
                Err(err) => {
                    web_sys::console::error_1(&format!("Parse error: {:?}", err).into());
                }
            }
            set_loading.set(false);
        });
    };

    // Load addresses on mount and when refresh_trigger changes
    Effect::new(move |_| {
        refresh_trigger.track();
        load_addresses();
    });

    let on_delete = move |_id: uuid::Uuid| {
        load_addresses();
    };

    view! {
        <div class="address-list">
            <h2>"주소 목록"</h2>
            
            <Show when=move || loading.get()>
                <div class="loading">"로딩 중..."</div>
            </Show>

            <Show when=move || !loading.get()>
                <div class="addresses">
                    <For
                        each=move || addresses.get()
                        key=|address| address.id
                        children=move |address| {
                            view! {
                                <AddressItem 
                                    address=address 
                                    on_delete=on_delete
                                />
                            }
                        }
                    />
                    
                    <Show when=move || addresses.get().is_empty()>
                        <div class="empty-state">
                            "등록된 주소가 없습니다."
                        </div>
                    </Show>
                </div>
            </Show>
        </div>
    }
}