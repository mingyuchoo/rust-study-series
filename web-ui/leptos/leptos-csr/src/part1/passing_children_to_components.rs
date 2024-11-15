use leptos::*;

#[component]
pub fn ComponentChildren() -> impl IntoView {
    view! {
        <h1>"Compoent Children"</h1>
        <TakesChildren render_prop=|| view! {<p>"Hi, there!"</p>}>
            // there get passed to `children`
           "Some text"
            <span>"A span"</span>
        </TakesChildren>
    }
}

#[component]
pub fn TakesChildren<F, IV>(// Take a function (type F) that returns anything
                            // that can be converted into a View (type IV)
                            render_prop: F,
                            // `children` takes the `Children` type
                            children: Children)
                            -> impl IntoView
    where F: Fn() -> IV,
          IV: IntoView,
{
    view! {
        <h2>"Render Prop"</h2>
        {render_prop()}
        <h2>"Children"</h2>
        {children()}
    }
}

#[component]
pub fn ManipulatingChildren() -> impl IntoView {
    view! {
        <h1>"Manipulating Children"</h1>
        <WrapsChildren>
           "A"
           "B"
           "C"
        </WrapsChildren>
    }
}

#[component]
pub fn WrapsChildren(children: Children) -> impl IntoView {
    // Fragment has `nodes` field that contains a Vec<View>
    let children = children().nodes
                             .into_iter()
                             .map(|child| view! {<li>{child}</li>})
                             .collect_view();
    view! {
        <h2>"WrapsChildren"</h2>
        <ul> {children} </ul>
    }
}
