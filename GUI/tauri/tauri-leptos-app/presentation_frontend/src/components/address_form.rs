use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::models::CreateAddressRequest;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn AddressForm<F>(on_save: F) -> impl IntoView 
where
    F: Fn() + 'static + Copy,
{
    let (name, set_name) = signal(String::new());
    let (phone, set_phone) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (street, set_street) = signal(String::new());
    let (city, set_city) = signal(String::new());
    let (postal_code, set_postal_code) = signal(String::new());
    let (country, set_country) = signal(String::new());
    let (is_saving, set_is_saving) = signal(false);

    let submit_form = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        
        if is_saving.get() {
            return;
        }

        set_is_saving.set(true);

        let request = CreateAddressRequest {
            name: name.get(),
            phone: phone.get(),
            email: email.get(),
            street: street.get(),
            city: city.get(),
            postal_code: postal_code.get(),
            country: country.get(),
        };

        spawn_local(async move {
            match serde_wasm_bindgen::to_value(&request) {
                Ok(args) => {
                    let result = invoke("create_address", args).await;
                    match serde_wasm_bindgen::from_value::<Result<crate::models::Address, String>>(result) {
                        Ok(Ok(_)) => {
                            // Clear form
                            set_name.set(String::new());
                            set_phone.set(String::new());
                            set_email.set(String::new());
                            set_street.set(String::new());
                            set_city.set(String::new());
                            set_postal_code.set(String::new());
                            set_country.set(String::new());
                            on_save();
                        }
                        Ok(Err(err)) => {
                            web_sys::console::error_1(&format!("Error: {}", err).into());
                        }
                        Err(err) => {
                            web_sys::console::error_1(&format!("Parse error: {:?}", err).into());
                        }
                    }
                }
                Err(err) => {
                    web_sys::console::error_1(&format!("Serialization error: {:?}", err).into());
                }
            }
            set_is_saving.set(false);
        });
    };

    view! {
        <div class="address-form">
            <h2>"새 주소 추가"</h2>
            <form on:submit=submit_form>
                <div class="form-group">
                    <label>"이름:"</label>
                    <input 
                        type="text"
                        prop:value=name
                        on:input=move |ev| set_name.set(event_target_value(&ev))
                        required
                    />
                </div>

                <div class="form-group">
                    <label>"전화번호:"</label>
                    <input 
                        type="tel"
                        prop:value=phone
                        on:input=move |ev| set_phone.set(event_target_value(&ev))
                        required
                    />
                </div>

                <div class="form-group">
                    <label>"이메일:"</label>
                    <input 
                        type="email"
                        prop:value=email
                        on:input=move |ev| set_email.set(event_target_value(&ev))
                        required
                    />
                </div>

                <div class="form-group">
                    <label>"주소:"</label>
                    <input 
                        type="text"
                        prop:value=street
                        on:input=move |ev| set_street.set(event_target_value(&ev))
                        required
                    />
                </div>

                <div class="form-group">
                    <label>"도시:"</label>
                    <input 
                        type="text"
                        prop:value=city
                        on:input=move |ev| set_city.set(event_target_value(&ev))
                        required
                    />
                </div>

                <div class="form-group">
                    <label>"우편번호:"</label>
                    <input 
                        type="text"
                        prop:value=postal_code
                        on:input=move |ev| set_postal_code.set(event_target_value(&ev))
                        required
                    />
                </div>

                <div class="form-group">
                    <label>"국가:"</label>
                    <input 
                        type="text"
                        prop:value=country
                        on:input=move |ev| set_country.set(event_target_value(&ev))
                        required
                    />
                </div>

                <button 
                    type="submit" 
                    class="btn btn-primary"
                    disabled=is_saving
                >
                    {move || if is_saving.get() { "저장 중..." } else { "저장" }}
                </button>
            </form>
        </div>
    }
}