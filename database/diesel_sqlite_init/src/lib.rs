pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use models::{NewPost, Post};
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).unwrap_or_else(|_| {
                                                  panic!("Error connecting to \
                                                          {}",
                                                         database_url)
                                              })
}

pub fn create_post(connection: &mut SqliteConnection,
                   title: &str,
                   body: &str)
                   -> Result<Post, diesel::result::Error> {
    // use self::schema::posts;
    use self::schema::posts::dsl::posts;

    let new_post = NewPost { title,
                             body };

    // diesel::insert_into(posts::table)
    diesel::insert_into(posts).values(&new_post)
                              .returning(Post::as_returning())
                              .get_result(connection)
}

pub fn select_post(connection: &mut SqliteConnection,
                   is_published: bool)
                   -> Result<Vec<Post>, diesel::result::Error> {
    use self::schema::posts::dsl::{posts, published};

    posts.filter(published.eq(is_published))
         .limit(5)
         .select(Post::as_select())
         .load(connection)
}

pub fn update_post(connection: &mut SqliteConnection,
                   id: i32)
                   -> Result<Post, diesel::result::Error> {
    use self::schema::posts::dsl::{posts, published};

    diesel::update(posts.find(id)).set(published.eq(true))
                                  .returning(Post::as_returning())
                                  .get_result(connection)
}

pub fn delete_post(connection: &mut SqliteConnection,
                   id: i32)
                   -> Result<usize, diesel::result::Error> {
    use self::schema::posts::dsl::posts;

    diesel::delete(posts.find(id)).execute(connection)
}
