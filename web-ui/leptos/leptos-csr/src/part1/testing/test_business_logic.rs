use leptos::*;

#[derive(Debug)]
pub struct Todo {
    pub title:     String,
    pub completed: bool,
}

impl Todo {
    pub fn new(title: &str,
               completed: bool)
               -> Self {
        Self { title:     title.to_string(),
               completed: completed, }
    }
}

#[component]
pub fn HardToTest() -> impl IntoView {
    let (todos, set_todos) = create_signal(vec![Todo::new("1", false),
                                                Todo::new("2", false),
                                                Todo::new("3", true),
                                                Todo::new("4", false),
                                                Todo::new("5", true)]);
    // FIX: this is hard to test because it's embedded in the component
    let num_remaining = move || {
        todos.with(|todos| {
                 todos.iter()
                      .filter(|todo| !todo.completed)
                      .count()
             })
    };

    view! {
        <main>
            <h1>"Testing"</h1>
            <h2>"Hard to test"</h2>
            <p>"Remaining: " {num_remaining()}</p>
            <ul>
                {
                    todos.with(|todos| {
                        todos.iter().map(|todo| view! {
                            <li>{todo.title.clone()}</li>
                        }).collect_view()
                    })
                }
            </ul>
        </main>
    }
}

pub struct Todos(Vec<Todo>);
impl Todos {
    pub fn num_remaining(&self) -> usize {
        self.0
            .iter()
            .filter(|todo| !todo.completed)
            .count()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_remaining() {
        //
    }
}
#[component]
pub fn EasyToTest() -> impl IntoView {
    let (todos, set_todos) = create_signal(Todos(vec![Todo::new("1", false),
                                                      Todo::new("2", false),
                                                      Todo::new("3", true),
                                                      Todo::new("4", false),
                                                      Todo::new("5", true)]));
    view! {}
}
