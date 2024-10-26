use diesel::prelude::*;
use diesel_postgres_init::*;
use std::env::args;

fn main() -> () {
    use self::schema::posts::dsl::*;

    let target: String = args().nth(1)
                               .expect("Expected a target to match against");
    let pattern: String = format!("%{}%", target);

    let connection: &mut PgConnection = &mut establish_connection();
    let num_deleted =
        diesel::delete(posts.filter(title.like(pattern))).execute(connection)
                                                         .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}
