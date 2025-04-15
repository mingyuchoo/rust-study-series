use crate::application::services::doc_application_service::DocApplicationService;
use crate::domain::services::doc_service::DocService;
use crate::domain::services::repositories::entities::doc::DocForm;
use crate::infrastructure::db::repositories::doc_db_repository::DocDbRepository;
use dioxus::prelude::*;
use std::path::Path;

const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn Documents() -> Element {
    // State for the document form
    let mut title = use_signal(|| String::new());
    let mut contents = use_signal(|| String::new());

    // State for the document list
    let mut documents = use_signal(Vec::new);
    let mut error = use_signal(|| Option::<String>::None);

    // State for editing
    let mut editing_id = use_signal(|| Option::<String>::None);

    // Initialize the repository and service
    // Create a proper absolute path for the database in the user's home directory
    let db_path = use_signal(|| match dirs::home_dir() {
        | Some(mut path) => {
            path.push(".local");
            path.push("share");
            path.push("my-dioxus-app");
            std::fs::create_dir_all(&path).unwrap_or_default();
            path.push("docs.db");
            path.to_str().unwrap_or("docs.db").to_string()
        },
        | None => "docs.db".to_string(),
    });

    // Function to load documents
    let mut load_documents = move || {
        let db_path_str = db_path();
        println!("Loading documents from database: {}", db_path_str);
        if let Ok(repo) = DocDbRepository::new(&db_path_str) {
            let doc_service = DocService::new(repo);
            let app_service = DocApplicationService::new(doc_service);

            match app_service.list_all_docs() {
                | Ok(docs) => {
                    documents.set(docs);
                    error.set(None);
                },
                | Err(err) => {
                    error.set(Some(format!("Error loading documents: {}", err)));
                },
            }
        } else {
            error.set(Some(format!("Failed to connect to database at {}", db_path)));
        }
    };

    // Load documents when component mounts
    use_effect(move || {
        load_documents();
        // Return empty cleanup function
        (|| {})()
    });

    // Handle form submission
    let handle_submit = move |event: FormEvent| {
        event.prevent_default();
        let db_path_str = db_path();
        println!("Submitting form to database: {}", db_path_str);

        if let Ok(repo) = DocDbRepository::new(&db_path_str) {
            let doc_service = DocService::new(repo);
            let app_service = DocApplicationService::new(doc_service);

            if let Some(id) = editing_id() {
                // Update existing document
                if let Some(doc) = app_service.get_doc_details(&id) {
                    let updated_form = DocForm {
                        title: title().clone(),
                        contents: contents().clone(),
                        archived: doc.archived,
                    };

                    if futures::executor::block_on(app_service.update(doc.id, updated_form)).is_ok() {
                        // Reset form
                        title.set(String::new());
                        contents.set(String::new());
                        editing_id.set(None);

                        // Reload documents
                        load_documents();
                    } else {
                        error.set(Some("Failed to update document".to_string()));
                    }
                }
            } else {
                // Create new document
                match app_service.register_doc(title().clone(), contents().clone()) {
                    | Ok(_) => {
                        // Reset form
                        title.set(String::new());
                        contents.set(String::new());

                        // Reload documents
                        load_documents();
                    },
                    | Err(err) => {
                        error.set(Some(format!("Error creating document: {}", err)));
                    },
                }
            }
        } else {
            error.set(Some(format!("Failed to connect to database at {}", db_path)));
        }
    };

    // Handle document deletion
    let handle_delete = move |id: String| {
        let db_path_str = db_path();
        if let Ok(repo) = DocDbRepository::new(&db_path_str) {
            let doc_service = DocService::new(repo);
            let app_service = DocApplicationService::new(doc_service);

            if app_service.delete_doc(&id).is_ok() {
                // Reload documents
                load_documents();
            } else {
                error.set(Some(format!("Failed to delete document with ID: {}", id)));
            }
        }
    };

    // Handle document editing
    let handle_edit = move |id: String| {
        let db_path_str = db_path();
        if let Ok(repo) = DocDbRepository::new(&db_path_str) {
            let doc_service = DocService::new(repo);
            let app_service = DocApplicationService::new(doc_service);

            if let Some(doc) = app_service.get_doc_details(&id) {
                // Set form values
                title.set(doc.title.clone());
                contents.set(doc.contents.clone());
                editing_id.set(Some(id));
            }
        }
    };

    // Handle form cancellation
    let handle_cancel = move |_| {
        title.set(String::new());
        contents.set(String::new());
        editing_id.set(None);
    };

    // Check if DB exists and show appropriate message
    let db_exists = Path::new(&db_path()).exists();

    rsx! {
        div {
            h1 { "Document Management" }

            // Error message
            {error().map(|err| rsx! {
                div {
                    p { "{err}" }
                }
            })}

            // DB status message
            {if !db_exists {
                rsx! {
                    div {
                        p { "Database file will be created automatically when you add your first document." }
                    }
                }
            } else { rsx!{} }}

            // Document form
            div {
                form { onsubmit: handle_submit,
                    h2 {
                        if editing_id().is_some() { "Edit Document" } else { "Add New Document" }
                    }

                    div {
                        label { "Title" }
                        input {
                            "type": "text",
                            placeholder: "Document title",
                            value: title,
                            oninput: move |evt| title.set(evt.value().clone()),
                            required: true
                        }
                    }

                    div {
                        label { "Contents" }
                        textarea {
                            placeholder: "Document contents",
                            value: contents,
                            oninput: move |evt| contents.set(evt.value().clone()),
                            rows: 5,
                            required: true
                        }
                    }

                    div {
                        button {
                            "type": "submit",
                            if editing_id().is_some() { "Update Document" } else { "Add Document" }
                        }

                        if editing_id().is_some() {
                            button {
                                "type": "button",
                                onclick: handle_cancel,
                                "Cancel"
                            }
                        }
                    }
                }
            }

            // Document list
            div {
                h2 { "Documents List" }

                if documents().is_empty() {
                    p { "No documents found." }
                } else {
                    table { class: "min-w-full bg-white",
                        thead { class: "bg-gray-100",
                            tr {
                                th { "ID" }
                                th { "Title" }
                                th { "Status" }
                                th { "Actions" }
                            }
                        }
                        tbody {
                            for doc in documents.peek().iter().cloned() {
                                tr { 
                                    td { "{doc.id}" }
                                    td { "{doc.title}" }
                                    td { class: "py-2 px-4 border-b",
                                        if doc.archived {
                                            span { "Archived" }
                                        } else {
                                            span { "Active" }
                                        }
                                    }
                                    td {
                                        div {
                                            button {
                                                onclick: move |_| handle_edit.clone()(doc.id.to_string()),
                                                "Edit"
                                            }
                                            button {
                                                onclick: move |_| handle_delete.clone()(doc.id.to_string()),
                                                "Delete"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
