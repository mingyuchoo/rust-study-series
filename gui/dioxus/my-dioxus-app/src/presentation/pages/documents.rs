use crate::application::services::doc_application_service::DocApplicationService;
use crate::domain::services::doc_service::DocService;
use crate::domain::services::repositories::entities::doc::DocForm;
use crate::infrastructure::db::repositories::doc_db_repository::DocDbRepository;
use dioxus::prelude::*;
use std::path::Path;

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
    let db_path = "docs.db";
    let _repo_result = DocDbRepository::new(db_path);
    
    // Function to load documents
    let mut load_documents = move || {
        if let Ok(repo) = DocDbRepository::new(db_path) {
            let doc_service = DocService::new(repo);
            let app_service = DocApplicationService::new(doc_service);
            
            match app_service.list_all_docs() {
                Ok(docs) => {
                    documents.set(docs);
                    error.set(None);
                }
                Err(err) => {
                    error.set(Some(format!("Error loading documents: {}", err)));
                }
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
        
        if let Ok(repo) = DocDbRepository::new(db_path) {
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
                    Ok(_) => {
                        // Reset form
                        title.set(String::new());
                        contents.set(String::new());
                        
                        // Reload documents
                        load_documents();
                    }
                    Err(err) => {
                        error.set(Some(format!("Error creating document: {}", err)));
                    }
                }
            }
        } else {
            error.set(Some(format!("Failed to connect to database at {}", db_path)));
        }
    };
    
    // Handle document deletion
    let mut handle_delete = move |id: String| {
        if let Ok(repo) = DocDbRepository::new(db_path) {
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
    let mut handle_edit = move |id: String| {
        if let Ok(repo) = DocDbRepository::new(db_path) {
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
    let db_exists = Path::new(db_path).exists();
    
    rsx! {
        div { class: "container mx-auto p-4",
            h1 { class: "text-2xl font-bold mb-4", "Document Management" }
            
            // Error message
            {error().map(|err| rsx! {
                div { class: "bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4",
                    p { "{err}" }
                }
            })}
            
            // DB status message
            {if !db_exists {
                rsx! {
                    div { class: "bg-yellow-100 border border-yellow-400 text-yellow-700 px-4 py-3 rounded mb-4",
                        p { "Database file will be created automatically when you add your first document." }
                    }
                }
            } else { rsx!{} }}
            
            // Document form
            div { class: "bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4",
                form { onsubmit: handle_submit,
                    h2 { class: "text-xl font-semibold mb-4", 
                        if editing_id().is_some() { "Edit Document" } else { "Add New Document" }
                    }
                    
                    div { class: "mb-4",
                        label { class: "block text-gray-700 text-sm font-bold mb-2", "Title" }
                        input {
                            class: "shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline",
                            "type": "text",
                            placeholder: "Document title",
                            value: title,
                            oninput: move |evt| title.set(evt.value().clone()),
                            required: true
                        }
                    }
                    
                    div { class: "mb-4",
                        label { class: "block text-gray-700 text-sm font-bold mb-2", "Contents" }
                        textarea {
                            class: "shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline",
                            placeholder: "Document contents",
                            value: contents,
                            oninput: move |evt| contents.set(evt.value().clone()),
                            rows: 5,
                            required: true
                        }
                    }
                    
                    div { class: "flex items-center justify-between",
                        button {
                            class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline",
                            "type": "submit",
                            if editing_id().is_some() { "Update Document" } else { "Add Document" }
                        }
                        
                        if editing_id().is_some() {
                            button {
                                class: "bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline",
                                "type": "button",
                                onclick: handle_cancel,
                                "Cancel"
                            }
                        }
                    }
                }
            }
            
            // Document list
            div { class: "bg-white shadow-md rounded px-8 pt-6 pb-8",
                h2 { class: "text-xl font-semibold mb-4", "Documents List" }
                
                if documents().is_empty() {
                    p { class: "text-gray-500 italic", "No documents found." }
                } else {
                    table { class: "min-w-full bg-white",
                        thead { class: "bg-gray-100",
                            tr {
                                th { class: "py-2 px-4 border-b text-left", "ID" }
                                th { class: "py-2 px-4 border-b text-left", "Title" }
                                th { class: "py-2 px-4 border-b text-left", "Status" }
                                th { class: "py-2 px-4 border-b text-left", "Actions" }
                            }
                        }
                        tbody {
                            for doc in documents.peek().iter().cloned() {
                                tr { class: "hover:bg-gray-50",
                                    td { class: "py-2 px-4 border-b", "{doc.id}" }
                                    td { class: "py-2 px-4 border-b", "{doc.title}" }
                                    td { class: "py-2 px-4 border-b", 
                                        if doc.archived {
                                            span { class: "text-red-500", "Archived" }
                                        } else {
                                            span { class: "text-green-500", "Active" }
                                        }
                                    }
                                    td { class: "py-2 px-4 border-b",
                                        div { class: "flex space-x-2",
                                            button {
                                                class: "bg-blue-500 hover:bg-blue-700 text-white text-sm py-1 px-2 rounded",
                                                onclick: move |_| handle_edit(doc.id.to_string()),
                                                "Edit"
                                            }
                                            button {
                                                class: "bg-red-500 hover:bg-red-700 text-white text-sm py-1 px-2 rounded",
                                                onclick: move |_| handle_delete(doc.id.to_string()),
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
