use crate::domain::entities::post::{Post, PostForm};
use crate::infrastructure::api::json_placeholder_api;
use dioxus::prelude::*;

#[component]
pub fn PostsTab() -> Element {
    let mut posts = use_signal(Vec::<Post>::new);
    let mut selected_post = use_signal(|| None::<Post>);
    let mut form = use_signal(PostForm::default);
    let mut is_editing = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);

    // Load posts on component mount
    use_effect(move || {
        spawn(async move {
            match json_placeholder_api::fetch_posts().await {
                | Ok(fetched_posts) => {
                    // Limit to first 20 posts for better performance
                    posts.set(fetched_posts.into_iter().take(20).collect());
                },
                | Err(err) => {
                    error.set(Some(format!("Error loading posts: {}", err)));
                },
            }
        });

        // Return empty cleanup function
        ()
    });

    let handle_create = move |_| {
        let form_data = form();
        let mut form_clone = form.clone();
        let mut posts_clone = posts.clone();
        let mut error_clone = error.clone();

        spawn(async move {
            match json_placeholder_api::create_post(form_data).await {
                | Ok(new_post) => {
                    posts_clone.write().push(new_post.clone());
                    form_clone.set(PostForm::default());
                    error_clone.set(None);
                },
                | Err(err) => {
                    error_clone.set(Some(format!("Error creating post: {}", err)));
                },
            }
        });
    };

    let handle_update = move |_| {
        if let Some(post) = selected_post() {
            let form_data = form();
            let mut form_clone = form.clone();
            let mut posts_clone = posts.clone();
            let mut selected_post_clone = selected_post.clone();
            let mut is_editing_clone = is_editing.clone();
            let mut error_clone = error.clone();

            spawn(async move {
                match json_placeholder_api::update_post(post.id, form_data).await {
                    | Ok(updated_post) => {
                        let mut posts_write = posts_clone.write();
                        if let Some(index) = posts_write.iter().position(|item| item.id == updated_post.id) {
                            posts_write[index] = updated_post.clone();
                        }
                        selected_post_clone.set(None);
                        form_clone.set(PostForm::default());
                        is_editing_clone.set(false);
                        error_clone.set(None);
                    },
                    | Err(err) => {
                        error_clone.set(Some(format!("Error updating post: {}", err)));
                    },
                }
            });
        }
    };

    let handle_delete = move |id: i32| {
        let mut posts_clone = posts.clone();
        let mut selected_post_clone = selected_post.clone();
        let mut form_clone = form.clone();
        let mut is_editing_clone = is_editing.clone();
        let mut error_clone = error.clone();

        spawn(async move {
            match json_placeholder_api::delete_post(id).await {
                | Ok(_) => {
                    posts_clone.write().retain(|post| post.id != id);
                    if selected_post_clone().map_or(false, |p| p.id == id) {
                        selected_post_clone.set(None);
                        form_clone.set(PostForm::default());
                        is_editing_clone.set(false);
                    }
                    error_clone.set(None);
                },
                | Err(err) => {
                    error_clone.set(Some(format!("Error deleting post: {}", err)));
                },
            }
        });
    };

    let mut handle_edit = move |post: Post| {
        selected_post.set(Some(post.clone()));
        form.set(PostForm {
            userId: post.userId,
            title: post.title,
            body: post.body,
        });
        is_editing.set(true);
    };

    let handle_cancel = move |_| {
        form.set(PostForm::default());
        is_editing.set(false);
    };

    rsx! {
        div { class: "p-4",
            h2 { class: "text-2xl font-bold mb-4", "Posts Management" }

            // Error message
            {error().map(|err| rsx!(
                div { class: "bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4",
                    p { {err} }
                }
            ))}

            // Post form
            div { class: "mb-6 p-4 border rounded",
                h3 { class: "text-xl font-semibold mb-2",
                    {if is_editing() { "Edit Post" } else { "Add New Post" }}
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

                    div { class: "mb-4",
                        label { class: "block text-sm font-medium text-gray-700", "Body" }
                        textarea {
                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500",
                            rows: "4",
                            value: form().body.clone(),
                            oninput: move |evt| {
                                let mut form_write = form.write();
                                form_write.body = evt.value().clone();
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
                                "Update Post"
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
                                "Add Post"
                            }
                        }
                    }}
                }
            }

            // Posts list
            div { class: "overflow-x-auto",
                table { class: "min-w-full bg-white border border-gray-300",
                    thead {
                        tr {
                            th { class: "py-2 px-4 border-b", "ID" }
                            th { class: "py-2 px-4 border-b", "User ID" }
                            th { class: "py-2 px-4 border-b", "Title" }
                            th { class: "py-2 px-4 border-b", "Actions" }
                        }
                    }
                    tbody {
                        {posts().into_iter().map(|post| {
                            let post_id = post.id;
                            let post_for_edit = post.clone();
                            rsx!(
                                tr { key: post.id.to_string(),
                                    td { class: "py-2 px-4 border-b", {post.id.to_string()} }
                                    td { class: "py-2 px-4 border-b", {post.userId.to_string()} }
                                    td { class: "py-2 px-4 border-b", {post.title.clone()} }
                                    td { class: "py-2 px-4 border-b",
                                        div { class: "flex space-x-2",
                                            button {
                                                class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-1 px-2 rounded text-sm",
                                                onclick: move |_| handle_edit(post_for_edit.clone()),
                                                "Edit"
                                            }
                                            button {
                                                class: "bg-red-500 hover:bg-red-700 text-white font-bold py-1 px-2 rounded text-sm",
                                                onclick: move |_| handle_delete(post_id),
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

            // Post detail view
            {selected_post().map(|post| rsx!(
                div { class: "mt-6 p-4 border rounded bg-gray-50",
                    h3 { class: "text-xl font-semibold mb-2", "Post Details" }
                    p { class: "font-bold", "Title: ", span { class: "font-normal", {post.title.clone()} } }
                    p { class: "font-bold", "Body: ", span { class: "font-normal", {post.body.clone()} } }
                }
            ))}
        }
    }
}
