use rocket::response::{status::Created, Debug};
use rocket::serde::json::Json;
use rocket::{get, post};

use rocket_dyn_templates::{context, Template};

use diesel::prelude::*;

/*
crate(main)
  |- schema
  |- database
  |- models
  |- controllers
*/
use crate::database;
use crate::models;
use crate::schema;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[post("/", format = "json", data = "<post>")]
pub fn post(post: Json<models::NewPost>) -> Result<Created<Json<models::NewPost>>> {
    use self::schema::posts::dsl::*;
    let mut connection = self::database::establish_connection_pg();

    let new_post = models::NewPost {
        title: post.title.to_string(),
        body: post.body.to_string(),
    };

    diesel::insert_into(posts)
        .values(&new_post)
        .execute(&mut connection)
        .expect("Error saving new post");

    Ok(Created::new("/").body(post))
}

#[get("/")]
pub fn get() -> Template {
    let connection = &mut self::database::establish_connection_pg();
    let results = self::schema::posts::dsl::posts
        .load::<models::Post>(connection)
        .expect("Error loading posts");
    Template::render("posts", context! {posts: &results, count: results.len()})
}
