wit_bindgen::generate!({
    world: "greeting-world",
    path: "wit",
});

struct GreetingComponent;

impl exports::component::greeting::greeter::Guest for GreetingComponent {
    fn greet(name: String) -> String {
        format!(
            "[WASI 0.2 Component] Hello, {}! Greetings from WebAssembly System Interface!",
            name
        )
    }

    fn get_version() -> String {
        String::from("greeting-component v0.1.0 (WASI 0.2)")
    }
}

export!(GreetingComponent);
