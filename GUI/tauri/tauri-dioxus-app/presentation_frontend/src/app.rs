#![allow(non_snake_case)]

use crate::components::{ContactForm, ContactFormData, ContactList};
use crate::models::{Contact, CreateContactRequest, UpdateContactRequest};
use crate::services::ContactService;
use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/styles.css");

#[derive(Debug, Clone, PartialEq)]
enum AppView {
    List,
    Add,
    Edit(Contact),
}

pub fn App() -> Element {
    let mut contacts = use_signal(Vec::<Contact>::new);
    let mut current_view = use_signal(|| AppView::List);
    let mut search_query = use_signal(String::new);
    let mut loading = use_signal(|| false);
    let mut error_message = use_signal(|| None::<String>);

    // Load contacts on app start
    use_effect(move || {
        spawn(async move {
            loading.set(true);
            match ContactService::list_contacts().await {
                | Ok(contact_list) => {
                    contacts.set(contact_list);
                    error_message.set(None);
                },
                | Err(e) => {
                    error_message.set(Some(format!("연락처를 불러오는데 실패했습니다: {}", e)));
                },
            }
            loading.set(false);
        });
    });

    let handle_search = move |_: FormEvent| {
        let query = search_query.read().clone();
        spawn(async move {
            loading.set(true);
            let result = if query.trim().is_empty() {
                ContactService::list_contacts().await
            } else {
                ContactService::search_contacts(query).await
            };

            match result {
                | Ok(contact_list) => {
                    contacts.set(contact_list);
                    error_message.set(None);
                },
                | Err(e) => {
                    error_message.set(Some(format!("검색에 실패했습니다: {}", e)));
                },
            }
            loading.set(false);
        });
    };

    let handle_add_contact = move |form_data: ContactFormData| {
        spawn(async move {
            loading.set(true);
            let request = CreateContactRequest {
                name: form_data.name,
                email: if form_data.email.is_empty() { None } else { Some(form_data.email) },
                phone: if form_data.phone.is_empty() { None } else { Some(form_data.phone) },
                address: if form_data.address.is_empty() { None } else { Some(form_data.address) },
            };

            match ContactService::create_contact(request).await {
                | Ok(_) => {
                    current_view.set(AppView::List);
                    if let Ok(contact_list) = ContactService::list_contacts().await {
                        contacts.set(contact_list);
                    }
                    error_message.set(None);
                },
                | Err(e) => {
                    error_message.set(Some(format!("연락처 추가에 실패했습니다: {}", e)));
                },
            }
            loading.set(false);
        });
    };

    let handle_edit_contact = move |form_data: ContactFormData| {
        if let AppView::Edit(contact) = current_view.read().clone() {
            spawn(async move {
                loading.set(true);
                let request = UpdateContactRequest {
                    id: contact.id,
                    name: Some(form_data.name),
                    email: Some(form_data.email),
                    phone: Some(form_data.phone),
                    address: Some(form_data.address),
                };

                match ContactService::update_contact(request).await {
                    | Ok(_) => {
                        current_view.set(AppView::List);
                        if let Ok(contact_list) = ContactService::list_contacts().await {
                            contacts.set(contact_list);
                        }
                        error_message.set(None);
                    },
                    | Err(e) => {
                        error_message.set(Some(format!("연락처 수정에 실패했습니다: {}", e)));
                    },
                }
                loading.set(false);
            });
        }
    };

    let handle_delete_contact = move |id: String| {
        spawn(async move {
            loading.set(true);
            match ContactService::delete_contact(id).await {
                | Ok(_) => {
                    if let Ok(contact_list) = ContactService::list_contacts().await {
                        contacts.set(contact_list);
                    }
                    error_message.set(None);
                },
                | Err(e) => {
                    error_message.set(Some(format!("연락처 삭제에 실패했습니다: {}", e)));
                },
            }
            loading.set(false);
        });
    };

    rsx! {
        link { rel: "stylesheet", href: CSS }
        main { class: "app",
            header { class: "app-header",
                h1 { "주소록" }

                if let AppView::List = current_view.read().clone() {
                    div { class: "header-actions",
                        form { class: "search-form", onsubmit: handle_search,
                            input {
                                r#type: "text",
                                placeholder: "연락처 검색...",
                                value: "{search_query}",
                                oninput: move |evt| search_query.set(evt.value())
                            }
                            button { r#type: "submit", "검색" }
                        }
                        button {
                            class: "btn btn-primary",
                            onclick: move |_| current_view.set(AppView::Add),
                            "새 연락처"
                        }
                    }
                }
            }

            if *loading.read() {
                div { class: "loading", "로딩 중..." }
            }

            if let Some(error) = error_message.read().clone() {
                div { class: "error-message", "{error}" }
            }

            match current_view.read().clone() {
                AppView::List => rsx! {
                    ContactList {
                        contacts: contacts.read().clone(),
                        on_edit: move |contact| current_view.set(AppView::Edit(contact)),
                        on_delete: handle_delete_contact
                    }
                },
                AppView::Add => rsx! {
                    ContactForm {
                        contact: None,
                        on_submit: handle_add_contact,
                        on_cancel: move |_| current_view.set(AppView::List)
                    }
                },
                AppView::Edit(contact) => rsx! {
                    ContactForm {
                        contact: Some(contact),
                        on_submit: handle_edit_contact,
                        on_cancel: move |_| current_view.set(AppView::List)
                    }
                }
            }
        }
    }
}
