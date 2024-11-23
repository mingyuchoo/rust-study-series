use diesel::prelude::*;
use diesel_postgres_init::*;
use std::io::{stdin, Read};

fn main() -> () {
    let connection: &mut PgConnection = &mut establish_connection();

    let mut title: String = String::new();
    let mut body: String = String::new();

    println!("What would you like your title to be?");
    stdin().read_line(&mut title)
           .unwrap();
    let title: &str = title.trim_end(); // Remove the trailing newline

    println!("\nOk! Let's write{} (Press {} when finished)\n", title, EOF);
    stdin().read_to_string(&mut body)
           .unwrap();

    let post: models::Post = create_post(connection, title, &body);
    println!("\nSaved draft {} with id {}", title, post.id);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(window)]
const EOF: &str = "CTRL+Z";
