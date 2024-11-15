use crate::services::posts;
use rocket::response::status::{Accepted,
                               Created};
use rocket::serde::json::Json;
use rocket::{get,
             post};

type PostsResult<T, E = posts::ServiceError> = Result<T, E>;

#[post("/posts", format = "json", data = "<post>")]
pub fn post(post: Json<posts::NewPost>) -> PostsResult<Created<Json<posts::NewPost>>> {
    Ok(Created::new("/").body(posts::create_post(post)))
}

#[get("/posts?<page>&<limit>")]
pub fn get(page: Option<i64>,
           limit: Option<i64>)
           -> PostsResult<Accepted<Json<Vec<posts::Post>>>> {
    let limit = limit.unwrap_or(10);
    let offset = (page.unwrap_or(1) - 1) * limit;
    Ok(Accepted(Json(posts::list_posts(offset, limit))))
}
