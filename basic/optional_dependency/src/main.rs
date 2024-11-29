#[cfg(feature = "json")]
use serde_json::json;

fn main() {
    let name = "Alice";
    let age = 30;

    #[cfg(feature = "json")]
    {
        let person = json!({
            "name": name,
            "age": age
        });

        println!("JSON output: {}", person.to_string());
    }
    #[cfg(not(feature = "json"))]
    {
        println!("Simple output: Name: {}, Age: {}", name, age);
    }
}
