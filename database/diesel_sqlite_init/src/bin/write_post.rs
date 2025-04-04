use diesel_sqlite_init::*;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection = &mut establish_connection();

    println!("What would you like your title to be?");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();

    let title = &title[.. (title.len() - 1)]; // Drop the newline character
    println!("\nOk! Let's write {title} (Press {EOF} when finished)\n");

    let mut body = String::new();
    stdin().read_to_string(&mut body).unwrap();

    let post =
        create_post(connection, title, &body).expect("Error saving new post");
    println!("\nSaved draft {title} with ID: {}", post.id);

    Ok(())
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
