use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::posts)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::posts)]
pub struct NewPost {
    pub title: String,
    pub body: String,
}

#[derive(Serialize, Debug)]
pub struct SingleDataResponse<T> {
    pub status: String,
    pub data: T,
}

#[derive(Serialize, Debug)]
pub struct MultiDataResponse<T> {
    pub status: String,
    pub data: Vec<T>,
}