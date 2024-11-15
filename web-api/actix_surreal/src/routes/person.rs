use crate::db::DB;
use crate::error::AppError;
use actix_web::web::{Json,
                     Path};
use actix_web::{delete,
                get,
                post,
                put};
use serde::{Deserialize,
            Serialize};
use surrealdb::RecordId;

const PERSON: &str = "person";

#[derive(Serialize, Deserialize)]
pub struct PersonData {
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Person {
    name: String,
    id:   RecordId,
}

#[post("/person/{id}")]
pub async fn create_person(id: Path<String>,
                           person: Json<PersonData>)
                           -> Result<Json<Option<Person>>, AppError> {
    let person = DB.create((PERSON, &*id))
                   .content(person)
                   .await?;
    Ok(Json(person))
}

#[get("/person/{id}")]
pub async fn read_person(id: Path<String>) -> Result<Json<Option<Person>>, AppError> {
    let person = DB.select((PERSON, &*id))
                   .await?;
    Ok(Json(person))
}

#[put("/person/{id}")]
pub async fn update_person(id: Path<String>,
                           person: Json<PersonData>)
                           -> Result<Json<Option<Person>>, AppError> {
    let person = DB.update((PERSON, &*id))
                   .content(person)
                   .await?;
    Ok(Json(person))
}

#[delete("/person/{id}")]
pub async fn delete_person(id: Path<String>) -> Result<Json<Option<Person>>, AppError> {
    let person = DB.delete((PERSON, &*id))
                   .await?;
    Ok(Json(person))
}

#[get("/people")]
pub async fn list_people() -> Result<Json<Vec<Person>>, AppError> {
    let people = DB.select(PERSON)
                   .await?;
    Ok(Json(people))
}
