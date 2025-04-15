use crate::application::services::user_application_service::UserApplicationService;
use crate::domain::services::repositories::entities::user::{User, UserForm};
use crate::domain::services::user_service::UserService;
use crate::infrastructure::api::jsonplaceholder_api_controller::UserApiController;
use crate::infrastructure::api::repositories::jsonplaceholder_api_repository::JsonPlaceholderUserRepository;
use dioxus::prelude::*;

#[component]
pub fn UsersTab() -> Element {
    let mut users = use_signal(Vec::<User>::new);
    let mut selected_user = use_signal(|| None::<User>);
    let mut form = use_signal(UserForm::default);
    let mut is_editing = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);

    // Load users on component mount
    use_effect(move || {
        spawn(async move {
            match {
                let repo = JsonPlaceholderUserRepository::new();
                let service = UserService::new(repo);
                let app_service = UserApplicationService::new(service);
                UserApiController::new(app_service)
            }
            .find_all()
            .await
            {
                | Ok(fetched_users) => {
                    users.set(fetched_users);
                },
                | Err(err) => {
                    error.set(Some(format!("Error loading users: {}", err)));
                },
            }
        });

        // Return empty cleanup function
    });

    let handle_create = move |_| {
        let form_data = form();
        let mut form_clone = form.clone();
        let mut users_clone = users.clone();
        let mut error_clone = error.clone();

        spawn(async move {
            match {
                let repo = JsonPlaceholderUserRepository::new();
                let service = UserService::new(repo);
                let app_service = UserApplicationService::new(service);
                UserApiController::new(app_service)
            }
            .create(form_data)
            .await
            {
                | Ok(new_user) => {
                    users_clone.write().push(new_user.clone());
                    form_clone.set(UserForm::default());
                    error_clone.set(None);
                },
                | Err(err) => {
                    error_clone.set(Some(format!("Error creating user: {}", err)));
                },
            }
        });
    };

    let handle_update = move |_| {
        if let Some(user) = selected_user() {
            let form_data = form();
            let mut users_clone = users;
            let mut selected_user_clone = selected_user;
            let mut form_clone = form;
            let mut is_editing_clone = is_editing;
            let mut error_clone = error;

            spawn(async move {
                match {
                    let repo = JsonPlaceholderUserRepository::new();
                    let service = UserService::new(repo);
                    let app_service = UserApplicationService::new(service);
                    UserApiController::new(app_service)
                }
                .update(user.id, form_data)
                .await
                {
                    | Ok(updated_user) => {
                        let mut users_write = users_clone.write();
                        if let Some(index) = users_write.iter().position(|item| item.id == updated_user.id) {
                            users_write[index] = updated_user.clone();
                        }
                        selected_user_clone.set(None);
                        form_clone.set(UserForm::default());
                        is_editing_clone.set(false);
                        error_clone.set(None);
                    },
                    | Err(err) => {
                        error_clone.set(Some(format!("Error updating user: {}", err)));
                    },
                }
            });
        }
    };

    let handle_delete = move |id: i32| {
        let mut users_clone = users;
        let mut selected_user_clone = selected_user;
        let mut form_clone = form;
        let mut is_editing_clone = is_editing;
        let mut error_clone = error;

        spawn(async move {
            match {
                let repo = JsonPlaceholderUserRepository::new();
                let service = UserService::new(repo);
                let app_service = UserApplicationService::new(service);
                UserApiController::new(app_service)
            }
            .delete(id)
            .await
            {
                | Ok(_) => {
                    users_clone.write().retain(|user| user.id != id);
                    if selected_user_clone().is_some_and(|u| u.id == id) {
                        selected_user_clone.set(None);
                        form_clone.set(UserForm::default());
                        is_editing_clone.set(false);
                    }
                    error_clone.set(None);
                },
                | Err(err) => {
                    error_clone.set(Some(format!("Error deleting user: {}", err)));
                },
            }
        });
    };

    let mut handle_edit = move |user: User| {
        selected_user.set(Some(user.clone()));
        form.set(UserForm {
            name: user.name,
            username: user.username,
            email: user.email,
            phone: user.phone.unwrap_or_default(),
        });
        is_editing.set(true);
    };

    let handle_cancel = move |_| {
        form.set(UserForm::default());
        is_editing.set(false);
    };

    rsx! {
        div { class: "p-4",
            h2 { class: "text-2xl font-bold mb-4", "Users Management" }

            // Error message
            {error().map(|err| rsx!(
                div { class: "bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4",
                    p { {err} }
                }
            ))}

            // User form
            div { class: "mb-6 p-4 border rounded",
                h3 { class: "text-xl font-semibold mb-2",
                    {if is_editing() { "Edit User" } else { "Add New User" }}
                }

                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                    div { class: "mb-4",
                        label { class: "block text-sm font-medium text-gray-700", "Name" }
                        input {
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500",
                            value: form().name.clone(),
                            oninput: move |evt| {
                                let mut form_write = form.write();
                                form_write.name = evt.value().clone();
                            }
                        }
                    }

                    div { class: "mb-4",
                        label { class: "block text-sm font-medium text-gray-700", "Username" }
                        input {
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500",
                            value: form().username.clone(),
                            oninput: move |evt| {
                                let mut form_write = form.write();
                                form_write.username = evt.value().clone();
                            }
                        }
                    }

                    div { class: "mb-4",
                        label { class: "block text-sm font-medium text-gray-700", "Email" }
                        input {
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500",
                            value: form().email.clone(),
                            oninput: move |evt| {
                                let mut form_write = form.write();
                                form_write.email = evt.value().clone();
                            }
                        }
                    }

                    div { class: "mb-4",
                        label { class: "block text-sm font-medium text-gray-700", "Phone" }
                        input {
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500",
                            value: form().phone.clone(),
                            oninput: move |evt| {
                                let mut form_write = form.write();
                                form_write.phone = evt.value().clone();
                            }
                        }
                    }
                }

                div { class: "flex space-x-2",
                    {if is_editing() {
                        rsx! {
                            button {
                                class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                                onclick: handle_update,
                                "Update User"
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
                                "Add User"
                            }
                        }
                    }}
                }
            }

            // Users list
            div { class: "overflow-x-auto",
                table { class: "min-w-full bg-white border border-gray-300",
                    thead {
                        tr {
                            th { class: "py-2 px-4 border-b", "ID" }
                            th { class: "py-2 px-4 border-b", "Name" }
                            th { class: "py-2 px-4 border-b", "Username" }
                            th { class: "py-2 px-4 border-b", "Email" }
                            th { class: "py-2 px-4 border-b", "Actions" }
                        }
                    }
                    tbody {
                        {users().into_iter().map(|user| {
                            let user_clone = user.clone();
                            let user_id = user.id;
                            rsx!(
                                tr { key: user.id.to_string(),
                                    td { class: "py-2 px-4 border-b", {user.id.to_string()} }
                                    td { class: "py-2 px-4 border-b", {user.name.clone()} }
                                    td { class: "py-2 px-4 border-b", {user.username.clone()} }
                                    td { class: "py-2 px-4 border-b", {user.email.clone()} }
                                    td { class: "py-2 px-4 border-b",
                                        div { class: "flex space-x-2",
                                            button {
                                                class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-1 px-2 rounded text-sm",
                                                onclick: move |_| handle_edit(user_clone.clone()),
                                                "Edit"
                                            }
                                            button {
                                                class: "bg-red-500 hover:bg-red-700 text-white font-bold py-1 px-2 rounded text-sm",
                                                onclick: move |_| handle_delete(user_id),
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
