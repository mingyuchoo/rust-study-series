use rocket::response::{status::Accepted, status::Created, Debug};
use rocket::serde::json::Json;
use rocket::{get, post};

use diesel::prelude::*;

use crate::database;
use crate::models::{NewPost, Post};
use crate::schema;

type DieselResult<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[post("/posts", format = "json", data = "<post>")]
pub fn post(post: Json<NewPost>) -> DieselResult<Created<Json<NewPost>>> {
    use self::schema::posts::dsl::posts;
    let mut connection = self::database::establish_connection_pg();

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

#[get("/posts?<page>&<limit>")]
pub fn get(page: Option<i64>, limit: Option<i64>) -> DieselResult<Accepted<Json<Vec<Post>>>> {
    let limit = limit.unwrap_or(10);
    let offset = (page.unwrap_or(1) -1) * limit;

    let connection = &mut self::database::establish_connection_pg();
    let results = self::schema::posts::dsl::posts
        .limit(limit)
        .offset(offset)
        .load::<Post>(connection)
        .expect("Error loading posts");

    Ok(Accepted(Json(results)))
}
