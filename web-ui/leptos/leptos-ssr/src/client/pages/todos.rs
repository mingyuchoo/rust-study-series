use crate::models::todo::*;
use crate::server::todo::fetch_todos;
use leptos::*;

#[component]
pub fn TodosPage() -> impl IntoView {
    let todos = create_resource(|| (), |_| fetch_todos());

    view! {
        <div class="todos-container">
            <h1>"Todo List"</h1>
            <Suspense
                fallback=move || view! { <p class="loading">"Loading..."</p> }
            >
                <TodoList todos/>
            </Suspense>
        </div>
    }
}

#[component]
fn TodoList(todos: Resource<(), Result<Vec<Todo>, ServerFnError>>)
            -> impl IntoView {
    move || {
        todos.get()
             .map(|result| match result {
                 | Ok(todos) => view! {
                     <div class="todo-list-container">
                         <ul class="todo-list">
                             <For
                                 each=move || todos.clone()
                                 key=|todo| todo.id
                                 children=move |todo| view! {
                                     <TodoItem todo/>
                                 }
                             />
                         </ul>
                     </div>
                 },
                 | Err(e) => view! {
                     <div class="todo-list-container">
                         <p class="error">"Error loading todos: " {e.to_string()}</p>
                     </div>
                 },
             })
    }
}

#[component]
fn TodoItem(todo: Todo) -> impl IntoView {
    view! {
        <li class="todo-item">
            <input
                type="checkbox"
                checked=todo.completed
                class="todo-checkbox"
            />
            <span class="todo-title">{todo.title}</span>
        </li>
    }
}
