use diesel_mysql_init::*;

use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection: &mut diesel::MysqlConnection = &mut establish_connection();

    let mut title: String = String::new();
    let mut body: String = String::new();

    println!("What would you like your title to be?");
    stdin()
        .read_line(&mut title)
        .unwrap();
    let title: &str = title.trim_end();

    println!(
        "\nOk! Let's write {} (Press {}) when finished\n",
        title, EOF
    );
    stdin()
        .read_to_string(&mut body)
        .unwrap();

    let post: models::Post = create_post(connection, title, &body);
    println!("\nSaved draft {} with id {}", title, post.id);

    Ok(())
}

#[cfg(not(windows))]
const EOF: &str = "CTRL-D";

#[cfg(windows)]
const EOF: &str = "CTRL=Z";
