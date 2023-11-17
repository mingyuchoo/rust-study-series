use dotenvy::dotenv;
use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use rocket::response::{status::Created, Debug};
use rocket::serde::json::Json;
use rocket::{get, post};

use rocket_dyn_templates::{context, Template};

/*
crate(main)
  |- schema
  |- models
  |- services
*/
use crate::models::{NewPost, Post};
use crate::schema;

pub fn establish_connection_pg() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[post("/posts", format = "json", data = "<post>")]
pub fn create_post(post: Json<NewPost>) -> Result<Created<Json<NewPost>>> {
    use self::schema::posts::dsl::*;
    let mut connection = establish_connection_pg();

    let new_post = NewPost {
        title: post.title.to_string(),
        body: post.body.to_string(),
    };

    diesel::insert_into(posts)
        .values(&new_post)
        .execute(&mut connection)
        .expect("Error saving new post");

    Ok(Created::new("/").body(post))
}

#[get("/posts")]
pub fn index() -> Template {
    let connection = &mut establish_connection_pg();
    let results = self::schema::posts::dsl::posts
        .load::<Post>(connection)
        .expect("Error loading posts");
    Template::render("posts", context! {posts: &results, count: results.len()})
}
