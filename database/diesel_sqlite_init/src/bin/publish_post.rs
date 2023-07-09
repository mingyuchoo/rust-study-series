use diesel_sqlite_init::*;
use std::env::args;

fn main() {
    let id = args()
        .nth(1)
        .expect("publish a post requires ID")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = &mut establish_connection();

    let post = update_post(connection, id).expect("Error updating post");

    println!("Published post - id: {}, title: {}", post.id, post.title);
}
