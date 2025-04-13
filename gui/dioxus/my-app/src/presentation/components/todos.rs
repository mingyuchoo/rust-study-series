use crate::infrastructure::api::json_placeholder_api;
use crate::domain::entities::todo::{Todo, TodoForm};
use dioxus::prelude::*;

#[component]
pub fn TodosTab() -> Element {
    let mut todos = use_signal(Vec::<Todo>::new);
    let mut selected_todo = use_signal(|| None::<Todo>);
    let mut form = use_signal(TodoForm::default);
    let mut is_editing = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);

    // Load todos on component mount
    use_effect(move || {
        spawn(async move {
            match json_placeholder_api::fetch_todos().await {
                | Ok(fetched_todos) => {
                    // Limit to first 20 todos for better performance
                    todos.set(fetched_todos.into_iter().take(20).collect());
                },
                | Err(err) => {
                    error.set(Some(format!("Error loading todos: {}", err)));
                },
            }
        });

        // Return empty cleanup function
        ()
    });

    let handle_create = move |_| {
        let form_data = form();
        let mut form_clone = form.clone();
        let mut todos_clone = todos.clone();
        let mut error_clone = error.clone();

        spawn(async move {
            match json_placeholder_api::create_todo(form_data).await {
                | Ok(new_todo) => {
                    todos_clone.write().push(new_todo.clone());
                    form_clone.set(TodoForm::default());
                    error_clone.set(None);
                },
                | Err(err) => {
                    error_clone.set(Some(format!("Error creating todo: {}", err)));
                },
            }
        });
    };

    let handle_update = move |_| {
        if let Some(todo) = selected_todo() {
            let form_data = form();
            let mut form_clone = form.clone();
            let mut todos_clone = todos.clone();
            let mut selected_todo_clone = selected_todo.clone();
            let mut is_editing_clone = is_editing.clone();
            let mut error_clone = error.clone();

            spawn(async move {
                match json_placeholder_api::update_todo(todo.id, form_data).await {
                    | Ok(updated_todo) => {
                        let mut todos_write = todos_clone.write();
                        if let Some(index) = todos_write.iter().position(|item| item.id == updated_todo.id) {
                            todos_write[index] = updated_todo.clone();
                        }
                        selected_todo_clone.set(None);
                        form_clone.set(TodoForm::default());
                        is_editing_clone.set(false);
                        error_clone.set(None);
                    },
                    | Err(err) => {
                        error_clone.set(Some(format!("Error updating todo: {}", err)));
                    },
                }
            });
        }
    };

    let handle_delete = move |id: i32| {
        let mut todos_clone = todos.clone();
        let mut selected_todo_clone = selected_todo.clone();
        let mut form_clone = form.clone();
        let mut is_editing_clone = is_editing.clone();
        let mut error_clone = error.clone();

        spawn(async move {
            match json_placeholder_api::delete_todo(id).await {
                | Ok(_) => {
                    todos_clone.write().retain(|todo| todo.id != id);
                    if selected_todo_clone().map_or(false, |t| t.id == id) {
                        selected_todo_clone.set(None);
                        form_clone.set(TodoForm::default());
                        is_editing_clone.set(false);
                    }
                    error_clone.set(None);
                },
                | Err(err) => {
                    error_clone.set(Some(format!("Error deleting todo: {}", err)));
                },
            }
        });
    };

    let mut handle_edit = move |todo: Todo| {
        selected_todo.set(Some(todo.clone()));
        form.set(TodoForm {
            userId: todo.userId,
            title: todo.title,
            completed: todo.completed,
        });
        is_editing.set(true);
    };

    let handle_cancel = move |_| {
        form.set(TodoForm::default());
        is_editing.set(false);
    };

    let toggle_completed = move |todo: Todo| {
        let mut todos_clone = todos.clone();
        let mut error_clone = error.clone();

        spawn(async move {
            let updated_form = TodoForm {
                userId: todo.userId,
                title: todo.title.clone(),
                completed: !todo.completed,
            };

            match json_placeholder_api::update_todo(todo.id, updated_form).await {
                | Ok(updated_todo) => {
                    let mut todos_write = todos_clone.write();
                    if let Some(index) = todos_write.iter().position(|item| item.id == updated_todo.id) {
                        todos_write[index] = updated_todo.clone();
                    }
                    error_clone.set(None);
                },
                | Err(err) => {
                    error_clone.set(Some(format!("Error updating todo: {}", err)));
                },
            }
        });
    };

    rsx! {
        div { class: "p-4",
            h2 { class: "text-2xl font-bold mb-4", "Todos Management" }

            // Error message
            {error().map(|err| rsx!(
                div { class: "bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4",
                    p { {err} }
                }
            ))}

            // Todo form
            div { class: "mb-6 p-4 border rounded",
                h3 { class: "text-xl font-semibold mb-2",
                    {if is_editing() { "Edit Todo" } else { "Add New Todo" }}
                }

                div { class: "grid grid-cols-1 gap-4",
                    div { class: "mb-4",
                        label { class: "block text-sm font-medium text-gray-700", "User ID" }
                        input {
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500",
                            r#type: "number",
                            value: form().userId.to_string(),
                            oninput: move |evt| {
                                let mut form_write = form.write();
                                if let Ok(id) = evt.value().parse::<i32>() {
                                    form_write.userId = id;
                                }
                            }
                        }
                    }

                    div { class: "mb-4",
                        label { class: "block text-sm font-medium text-gray-700", "Title" }
                        input {
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500",
                            value: form().title.clone(),
                            oninput: move |evt| {
                                let mut form_write = form.write();
                                form_write.title = evt.value().clone();
                            }
                        }
                    }

                    div { class: "mb-4 flex items-center",
                        input {
                            id: "completed",
                            class: "h-4 w-4 text-indigo-600 focus:ring-indigo-500 border-gray-300 rounded",
                            r#type: "checkbox",
                            checked: form().completed,
                            oninput: move |evt| {
                                let mut form_write = form.write();
                                form_write.completed = evt.value().parse().unwrap_or(false);
                            }
                        }
                        label { class: "ml-2 block text-sm text-gray-900", r#for: "completed", "Completed" }
                    }
                }

                div { class: "flex space-x-2",
                    {if is_editing() {
                        rsx! {
                            button {
                                class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                                onclick: handle_update,
                                "Update Todo"
                            }
                            button {
                                class: "bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded",
                                onclick: handle_cancel,
                                "Cancel"
                            }
                        }
                    } else {
                        rsx! {
                            button {
                                class: "bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded",
                                onclick: handle_create,
                                "Add Todo"
                            }
                        }
                    }}
                }
            }

            // Todos list
            div { class: "overflow-x-auto",
                table { class: "min-w-full bg-white border border-gray-300",
                    thead {
                        tr {
                            th { class: "py-2 px-4 border-b", "ID" }
                            th { class: "py-2 px-4 border-b", "User ID" }
                            th { class: "py-2 px-4 border-b", "Title" }
                            th { class: "py-2 px-4 border-b", "Status" }
                            th { class: "py-2 px-4 border-b", "Actions" }
                        }
                    }
                    tbody {
                        {todos().into_iter().map(|todo| {
                            let todo_id = todo.id;
                            let todo_for_toggle = todo.clone();
                            let todo_for_edit = todo.clone();
                            rsx!(
                                tr { key: todo.id.to_string(),
                                    td { class: "py-2 px-4 border-b", {todo.id.to_string()} }
                                    td { class: "py-2 px-4 border-b", {todo.userId.to_string()} }
                                    td { class: "py-2 px-4 border-b", {todo.title.clone()} }
                                    td { class: "py-2 px-4 border-b",
                                        div { class: "flex items-center",
                                            input {
                                                class: "h-4 w-4 text-indigo-600 focus:ring-indigo-500 border-gray-300 rounded",
                                                r#type: "checkbox",
                                                checked: todo.completed,
                                                onclick: move |_| toggle_completed(todo_for_toggle.clone())
                                            }
                                            span { class: "ml-2",
                                                {if todo.completed { "Completed" } else { "Pending" }}
                                            }
                                        }
                                    }
                                    td { class: "py-2 px-4 border-b",
                                        div { class: "flex space-x-2",
                                            button {
                                                class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-1 px-2 rounded text-sm",
                                                onclick: move |_| handle_edit(todo_for_edit.clone()),
                                                "Edit"
                                            }
                                            button {
                                                class: "bg-red-500 hover:bg-red-700 text-white font-bold py-1 px-2 rounded text-sm",
                                                onclick: move |_| handle_delete(todo_id),
                                                "Delete"
                                            }
                                        }
                                    }
                                }
                            )
                        })}
                    }
                }
            }
        }
    }
}
