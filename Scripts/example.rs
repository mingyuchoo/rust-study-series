#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! serde = {version = "1.0", features = ["derive"]}
//! serde_json = "1.0"
//! reqwest = {version = "0.11", features = ["blocking"]}
//! tokio = {version = "1", features = ["full"]}
//! ```

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person = Person {
        name: "Adam".to_string(),
        age: 30,
    };

    let json = serde_json::to_string(&person).unwrap();
    println!("{}", json);
}
