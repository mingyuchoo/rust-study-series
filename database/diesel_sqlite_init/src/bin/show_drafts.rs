use diesel_sqlite_init::*;

fn main() {
    let connection = &mut establish_connection();

    let results = select_post(connection, false).expect("Error loading drafts.");

    println!("Displaying {} drafts", results.len());

    results.iter().for_each(|post| {
        println!("----------------------------");
        println!("ID: {:<5}Title: {:<10}", post.id, post.title);
        println!("============================");
        println!("{}", post.body);
    });
}
