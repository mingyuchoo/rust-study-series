use crate::domain::entities::*;
use crate::services::ContactApi;
use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn App() -> impl IntoView {
    let (contacts, set_contacts) = signal(Vec::<Contact>::new());
    let (search_query, set_search_query) = signal(String::new());
    let (is_loading, set_is_loading) = signal(false);
    let (error, set_error) = signal(None::<String>);
    let (show_form, set_show_form) = signal(false);
    let (editing_contact, set_editing_contact) = signal(None::<Contact>);

    // Load contacts on mount
    Effect::new(move |_| {
        set_is_loading.set(true);
        spawn_local(async move {
            match ContactApi::get_all_contacts().await {
                | Ok(contacts_list) => {
                    set_contacts.set(contacts_list);
                    set_error.set(None);
                },
                | Err(err) => {
                    set_error.set(Some(err.to_string()));
                },
            }
            set_is_loading.set(false);
        });
    });

    let filtered_contacts = move || {
        let query = search_query.get().to_lowercase();
        if query.is_empty() {
            contacts.get()
        } else {
            contacts
                .get()
                .into_iter()
                .filter(|contact| {
                    contact.name.to_lowercase().contains(&query)
                        || contact.email.as_ref().is_some_and(|e| e.to_lowercase().contains(&query))
                        || contact.phone.as_ref().is_some_and(|p| p.contains(&query))
                })
                .collect()
        }
    };

    view! {
        <div class="app">
            <header class="app-header">
                <h1 class="app-title">"주소록"</h1>
                <button
                    class="btn btn-primary"
                    on:click=move |_| {
                        set_editing_contact.set(None);
                        set_show_form.set(true);
                    }
                >
                    "+ 새 연락처"
                </button>
            </header>

            {move || error.get().map(|err| view! {
                <div class="error-banner">
                    <span>{err}</span>
                    <button class="error-close" on:click=move |_| set_error.set(None)>"×"</button>
                </div>
            })}

            <main class="app-main">
                {move || if show_form.get() {
                    view! {
                        <div class="contact-form-container">
                            <div class="contact-form-card">
                                <h2 class="form-title">
                                    {if editing_contact.get().is_some() { "연락처 수정" } else { "새 연락처 추가" }}
                                </h2>
                                <ContactFormInner
                                    contact=editing_contact.get()
                                    _contacts=contacts
                                    set_contacts=set_contacts
                                    set_show_form=set_show_form
                                    set_error=set_error
                                />
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div>
                            <div class="search-bar">
                                <input
                                    type="text"
                                    class="search-input"
                                    placeholder="이름, 이메일, 전화번호로 검색..."
                                    prop:value=search_query
                                    on:input=move |ev| set_search_query.set(event_target_value(&ev))
                                />
                            </div>

                            {move || if is_loading.get() {
                                view! {
                                    <div class="loading">
                                        <div class="loading-spinner"></div>
                                        <p>"연락처를 불러오는 중..."</p>
                                    </div>
                                }.into_any()
                            } else {
                                let contacts_list = filtered_contacts();
                                view! {
                                    <div class="contact-list">
                                        {if contacts_list.is_empty() {
                                            view! {
                                                <div class="empty-state">
                                                    <div class="empty-icon">"📱"</div>
                                                    <h3>"연락처가 없습니다"</h3>
                                                    <p>"새 연락처를 추가해보세요."</p>
                                                </div>
                                            }.into_any()
                                        } else {
                                            contacts_list.into_iter().map(|contact| {
                                                let contact_for_edit = contact.clone();
                                                let contact_id = contact.id;

                                                view! {
                                                    <div class="contact-card">
                                                        <div class="contact-info">
                                                            <div class="contact-avatar">
                                                                {contact.name.chars().next().unwrap_or('?').to_uppercase().to_string()}
                                                            </div>
                                                            <div class="contact-details">
                                                                <h3 class="contact-name">{contact.name.clone()}</h3>
                                                                {contact.email.as_ref().map(|email| view! {
                                                                    <p class="contact-email">{email.clone()}</p>
                                                                })}
                                                                {contact.phone.as_ref().map(|phone| view! {
                                                                    <p class="contact-phone">{phone.clone()}</p>
                                                                })}
                                                            </div>
                                                        </div>
                                                        <div class="contact-actions">
                                                            <button
                                                                class="btn-icon btn-edit"
                                                                on:click=move |_| {
                                                                    set_editing_contact.set(Some(contact_for_edit.clone()));
                                                                    set_show_form.set(true);
                                                                }
                                                                title="수정"
                                                            >
                                                                "✏️"
                                                            </button>
                                                            <button
                                                                class="btn-icon btn-delete"
                                                                on:click=move |_| {
                                                                    if web_sys::window()
                                                                        .unwrap()
                                                                        .confirm_with_message("정말로 삭제하시겠습니까?")
                                                                        .unwrap_or(false)
                                                                    {
                                                                        spawn_local(async move {
                                                                            match ContactApi::delete_contact(&contact_id.to_string()).await {
                                                                                Ok(_) => {
                                                                                    match ContactApi::get_all_contacts().await {
                                                                                        Ok(contacts_list) => {
                                                                                            set_contacts.set(contacts_list);
                                                                                        }
                                                                                        Err(err) => {
                                                                                            set_error.set(Some(err.to_string()));
                                                                                        }
                                                                                    }
                                                                                }
                                                                                Err(err) => {
                                                                                    set_error.set(Some(err.to_string()));
                                                                                }
                                                                            }
                                                                        });
                                                                    }
                                                                }
                                                                title="삭제"
                                                            >
                                                                "🗑️"
                                                            </button>
                                                        </div>
                                                    </div>
                                                }
                                            }).collect::<Vec<_>>().into_any()
                                        }}
                                    </div>
                                }.into_any()
                            }}
                        </div>
                    }.into_any()
                }}
            </main>
        </div>
    }
}

#[component]
pub fn ContactFormInner(
    contact: Option<Contact>,
    _contacts: ReadSignal<Vec<Contact>>,
    set_contacts: WriteSignal<Vec<Contact>>,
    set_show_form: WriteSignal<bool>,
    set_error: WriteSignal<Option<String>>,
) -> impl IntoView {
    let (name, set_name) = signal(contact.as_ref().map(|c| c.name.clone()).unwrap_or_default());
    let (email, set_email) = signal(contact.as_ref().and_then(|c| c.email.clone()).unwrap_or_default());
    let (phone, set_phone) = signal(contact.as_ref().and_then(|c| c.phone.clone()).unwrap_or_default());
    let (address, set_address) = signal(contact.as_ref().and_then(|c| c.address.clone()).unwrap_or_default());
    let (form_error, set_form_error) = signal(None::<String>);
    let (is_loading, set_is_loading) = signal(false);

    let is_edit_mode = contact.is_some();

    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();

        let name_val = name.get();
        let email_val = email.get();
        let phone_val = phone.get();
        let address_val = address.get();

        if name_val.trim().is_empty() {
            set_form_error.set(Some("이름은 필수입니다.".to_string()));
            return;
        }

        set_is_loading.set(true);
        set_form_error.set(None);

        let contact_clone = contact.clone();
        spawn_local(async move {
            let result = if let Some(existing_contact) = &contact_clone {
                let request = UpdateContactRequest {
                    name: if name_val != existing_contact.name { Some(name_val) } else { None },
                    email: if email_val != existing_contact.email.clone().unwrap_or_default() {
                        Some(email_val)
                    } else {
                        None
                    },
                    phone: if phone_val != existing_contact.phone.clone().unwrap_or_default() {
                        Some(phone_val)
                    } else {
                        None
                    },
                    address: if address_val != existing_contact.address.clone().unwrap_or_default() {
                        Some(address_val)
                    } else {
                        None
                    },
                };
                ContactApi::update_contact(&existing_contact.id.to_string(), request).await
            } else {
                let request = CreateContactRequest {
                    name: name_val,
                    email: if email_val.is_empty() { None } else { Some(email_val) },
                    phone: if phone_val.is_empty() { None } else { Some(phone_val) },
                    address: if address_val.is_empty() { None } else { Some(address_val) },
                };
                ContactApi::create_contact(request).await
            };

            set_is_loading.set(false);

            match result {
                | Ok(_) => {
                    // Reload all contacts
                    match ContactApi::get_all_contacts().await {
                        | Ok(contacts_list) => {
                            set_contacts.set(contacts_list);
                            set_show_form.set(false);
                        },
                        | Err(err) => {
                            set_error.set(Some(err.to_string()));
                        },
                    }
                },
                | Err(err) => {
                    set_form_error.set(Some(err.to_string()));
                },
            }
        });
    };

    view! {
        <form on:submit=handle_submit class="contact-form">
            <div class="form-group">
                <label for="name" class="form-label">"이름 *"</label>
                <input
                    type="text"
                    id="name"
                    class="form-input"
                    placeholder="이름을 입력하세요"
                    prop:value=name
                    on:input=move |ev| set_name.set(event_target_value(&ev))
                    required
                />
            </div>

            <div class="form-group">
                <label for="email" class="form-label">"이메일"</label>
                <input
                    type="email"
                    id="email"
                    class="form-input"
                    placeholder="이메일을 입력하세요"
                    prop:value=email
                    on:input=move |ev| set_email.set(event_target_value(&ev))
                />
            </div>

            <div class="form-group">
                <label for="phone" class="form-label">"전화번호"</label>
                <input
                    type="tel"
                    id="phone"
                    class="form-input"
                    placeholder="전화번호를 입력하세요"
                    prop:value=phone
                    on:input=move |ev| set_phone.set(event_target_value(&ev))
                />
            </div>

            <div class="form-group">
                <label for="address" class="form-label">"주소"</label>
                <textarea
                    id="address"
                    class="form-textarea"
                    placeholder="주소를 입력하세요"
                    prop:value=address
                    on:input=move |ev| set_address.set(event_target_value(&ev))
                />
            </div>

            {move || form_error.get().map(|err| view! {
                <div class="error-message">
                    {err}
                </div>
            })}

            <div class="form-actions">
                <button
                    type="button"
                    class="btn btn-secondary"
                    on:click=move |_| set_show_form.set(false)
                    disabled=is_loading
                >
                    "취소"
                </button>
                <button
                    type="submit"
                    class="btn btn-primary"
                    disabled=is_loading
                >
                    {move || if is_loading.get() {
                        if is_edit_mode { "수정 중..." } else { "저장 중..." }
                    } else if is_edit_mode { "수정" } else { "저장" }}
                </button>
            </div>
        </form>
    }
}
