use persistence::*;
use std::io::{stdin, Read};

fn main() -> () {
    let connection = &mut establish_connection();

    let mut title: String = String::new();
    let mut body: String = String::new();

    println!("What would you like your title to be?");
    stdin().read_line(&mut title).unwrap();
    let title: &str = title.trim_end(); // Remove the trailing newline

    println!("Ok! Let's write {} (Press {} when finished)", title, EOF);
    stdin().read_to_string(&mut body).unwrap();

    let todo = create_todo(connection, title);
    println!("\nSaved draft {} with id {}", title, todo.id);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
