#![allow(non_snake_case)]

use crate::models::Contact;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ContactListProps {
    pub contacts: Vec<Contact>,
    pub on_edit: EventHandler<Contact>,
    pub on_delete: EventHandler<String>,
}

pub fn ContactList(props: ContactListProps) -> Element {
    rsx! {
        div { class: "contact-list",
            if props.contacts.is_empty() {
                div { class: "empty-state",
                    p { "연락처가 없습니다." }
                }
            } else {
                div { class: "contacts-grid",
                    for contact in props.contacts.iter() {
                        ContactCard {
                            contact: contact.clone(),
                            on_edit: props.on_edit,
                            on_delete: props.on_delete
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ContactCardProps {
    pub contact: Contact,
    pub on_edit: EventHandler<Contact>,
    pub on_delete: EventHandler<String>,
}

pub fn ContactCard(props: ContactCardProps) -> Element {
    let contact = props.contact.clone();

    rsx! {
        div { class: "contact-card",
            div { class: "contact-header",
                h3 { "{contact.name}" }
                div { class: "contact-actions",
                    button {
                        class: "btn btn-sm btn-outline",
                        onclick: {
                            let contact = contact.clone();
                            move |_| props.on_edit.call(contact.clone())
                        },
                        "수정"
                    }
                    button {
                        class: "btn btn-sm btn-danger",
                        onclick: {
                            let contact_id = contact.id.clone();
                            move |_| props.on_delete.call(contact_id.clone())
                        },
                        "삭제"
                    }
                }
            }

            div { class: "contact-details",
                if let Some(email) = &contact.email {
                    if !email.is_empty() {
                        div { class: "contact-field",
                            span { class: "field-label", "이메일:" }
                            span { class: "field-value", "{email}" }
                        }
                    }
                }

                if let Some(phone) = &contact.phone {
                    if !phone.is_empty() {
                        div { class: "contact-field",
                            span { class: "field-label", "전화:" }
                            span { class: "field-value", "{phone}" }
                        }
                    }
                }

                if let Some(address) = &contact.address {
                    if !address.is_empty() {
                        div { class: "contact-field",
                            span { class: "field-label", "주소:" }
                            span { class: "field-value", "{address}" }
                        }
                    }
                }
            }
        }
    }
}
