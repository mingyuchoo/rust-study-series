use diesel_sqlite_init::*;
use std::env::args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let id = args().nth(1)
                   .expect("Delete a post requires ID")
                   .parse::<i32>()
                   .expect("Invalid ID");

    let connection = &mut establish_connection();

    let num_deleted =
        delete_post(connection, id).expect("Error deleting a post.");

    println!("Deleted {num_deleted} posts");

    Ok(())
}
