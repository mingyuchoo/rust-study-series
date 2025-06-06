pub mod models;
pub mod schema;

use self::models::{NewPost, Post};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url: String =
        env::var("DATABASE_URL").expect("DATBASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connection to {database_url}"))
}

pub fn create_post(conn: &mut PgConnection, title: &str, body: &str) -> Post {
    use self::schema::posts;

    let new_post: NewPost<'_> = NewPost {
        title,
        body,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}
