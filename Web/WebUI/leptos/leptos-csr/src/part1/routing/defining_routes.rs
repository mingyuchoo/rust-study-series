use leptos::*;
use leptos_router::*;

#[component]
pub fn DefiningRoutes() -> impl IntoView {
    view! {
        <Router>
            <h1>"Contact App"</h1>
            // this <nav> will show on every routes,
            // because it's outside the <Routes/>
            // note: we can just use normal <a> tags
            // and the router will use client-side navigation
            <nav>
                <h2>"Navigation"</h2>
                <a href="/">"Home"</a>
                <a href="/contact">"Contacts"</a>
            </nav>
            <main>
                <Routes>
                    <Route path="/" view=|| view! {<h3>"Home"</h3>}/>
                    <Route path="/contacts" view=ContactList>
                        <Route path=":id" view=ContactInfo>
                            <Route path="" view=|| view! {<div class="tab">"(Contact Info)"</div>}/>
                            <Route path="conversations" view=|| view! {<div class="tab">"(Conversations)"</div>}/>
                        </Route>
                        <Route path="" view=|| view! {<div class="select-user">"Select a user to view contact info."</div>}/>
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn ContactList() -> impl IntoView {
    view! {
        <div class="contact-list">
            <div class="contact-list-contacts">
                <h3>"Contacts"</h3>
                <A href="alice">"Alice"</A>
                <A href="bob">"Bob"</A>
                <A href="steve">"Steve"</A>
            </div>

            // <Outlet/> will show the nested child route
            // we can position this outlet wherever we want
            // within the layout
            <Outlet/>
        </div>
    }
}

#[component]
fn ContactInfo() -> impl IntoView {
    // we can access the :id param reactively with `use_params_map`
    let params = use_params_map();
    let id = move || {
        params.with(|params| {
                  params.get("id")
                        .cloned()
                        .unwrap_or_default()
              })
    };

    // image we're loading data from an API here
    let name = move || match id().as_str() {
        | "alice" => "Alice",
        | "bob" => "Bob",
        | "steve" => "Steve",
        | _ => "User not found.",
    };

    view! {
        <div class="contact-info">
            <h4>{name}</h4>
            <div class="tabs">
                <A href="" exact=true>"Contact Info"</A>
                <A href="conversations">"Conversations"</A>
            </div>

            // <Outlet/> here is the tabs that are nested
            // underneath the /contacs/:id route
            <Outlet/>
        </div>
    }
}
