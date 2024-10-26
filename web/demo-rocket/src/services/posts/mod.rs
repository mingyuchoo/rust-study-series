use rocket::serde::{json::Json,
                    Deserialize,
                    Serialize};

use diesel::{prelude::*,
             result,
             Insertable,
             Queryable};
use rocket::response::Debug;

use crate::{database,
            schema};

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::posts)]
pub struct Post
{
    pub id:        i32,
    pub title:     String,
    pub body:      String,
    pub published: bool,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::posts)]
pub struct NewPost
{
    pub title: String,
    pub body:  String,
}

pub type ServiceError = Debug<result::Error>;

pub fn create_post(post: Json<NewPost>) -> Json<NewPost>
{
    use schema::posts::dsl::posts;
    let mut connection = database::establish_connection_pg();
    let new_post = NewPost { title: post.title
                                        .to_owned(),
                             body:  post.body
                                        .to_owned(), };
    diesel::insert_into(posts).values(&new_post)
                              .execute(&mut connection)
                              .expect("Error saving new post");
    post
}

pub fn list_posts(offset: i64,
                  limit: i64)
                  -> Vec<Post>
{
    let connection: &mut PgConnection = &mut database::establish_connection_pg();
    schema::posts::dsl::posts.limit(limit)
                             .offset(offset)
                             .load::<Post>(connection)
                             .expect("Error loading posts")
}
