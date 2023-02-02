pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use self::models::{NewPost, Post};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_post(conn: &mut PgConnection, title: &str, body: &str, published: bool) -> Post {
    use crate::schema::posts;
    let new_post = NewPost {
        title,
        body,
        published,
    };
    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn delete_post(conn: &mut PgConnection, pattern: &str) -> usize {
    use crate::schema::posts::dsl::*;
    diesel::delete(posts.filter(title.like(pattern)))
        .execute(conn)
        .expect("Error deleting posts")
}

pub fn update_post(conn: &mut PgConnection, id: i32) -> Post {
    use crate::schema::posts::dsl::*;
    diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result::<Post>(conn)
        .unwrap()
}

pub fn show_posts(conn: &mut PgConnection) {
    use crate::schema::posts::dsl::*;
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(conn)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());

    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}
