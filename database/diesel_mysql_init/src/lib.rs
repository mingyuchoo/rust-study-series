pub mod models;
pub mod schema;

use self::models::{NewPost, Post};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url: String =
        env::var("DATABASE_URL").expect("DATBASE_URL mut be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connection to {}", database_url))
}

pub fn create_post(
    conn: &mut MysqlConnection,
    title: &str,
    body: &str,
) -> Post {
    use crate::schema::posts;

    let new_post: NewPost<'_> = NewPost {
        title,
        body,
    };

    conn.transaction(|conn: &mut MysqlConnection| {
        diesel::insert_into(posts::table)
            .values(&new_post)
            .execute(conn)?;
        posts::table
            .order(posts::id.desc())
            .select(Post::as_select())
            .first(conn)
    })
    .expect("Error while saving post")
}
