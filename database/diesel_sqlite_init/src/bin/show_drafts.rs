use self::models::*;
use diesel::prelude::*;
use diesel_sqlite_init::*;

fn main() {
    use self::schema::posts::dsl::*;

    let connection = &mut establish_connection();

    let results = posts
        .filter(published.eq(false))
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading drafts");

    println!("Displaying {} drafts", results.len());

    results.iter().for_each(|post| {
        println!("----------------------------");
        println!("ID: {:<5}Title: {:<10}", post.id, post.title);
        println!("============================");
        println!("{}", post.body);
    });
}
