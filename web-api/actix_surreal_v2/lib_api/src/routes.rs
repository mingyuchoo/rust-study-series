use crate::error::Error;
use actix_web::web::{Json, Path};
use actix_web::{delete, get, post, put};
use serde::{Deserialize, Serialize};
use lib_db::{Id, DB};

#[derive(Serialize, Deserialize)]
pub struct Person {
    id:   Id,
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct PersonParam {
    name: String,
}

#[get("/session")]
pub async fn session() -> Result<Json<String>, Error> {
    let res: Option<String> = DB.query("RETURN <string>$session")
                                .await?
                                .take(0)?;

    Ok(Json(res.unwrap_or("No session data found!".into())))
}

#[get("/people")]
pub async fn list_people() -> Result<Json<Vec<Person>>, Error> {
    let people = DB.select("person")
                   .await?;
    Ok(Json(people))
}

#[post("/person")]
pub async fn create_person(person: Json<PersonParam>)
                           -> Result<Json<Option<Person>>, Error> {
    let person = DB.create("person")
                   .content(person)
                   .await?;
    Ok(Json(person))
}

#[get("/person/{id}")]
pub async fn read_person(id: Path<String>)
                         -> Result<Json<Option<Person>>, Error> {
    let person = DB.select(("person", &*id))
                   .await?;
    Ok(Json(person))
}

#[put("/person/{id}")]
pub async fn update_person(id: Path<String>,
                           person: Json<PersonParam>)
                           -> Result<Json<Option<Person>>, Error> {
    let person = DB.update(("person", &*id))
                   .content(person)
                   .await?;
    Ok(Json(person))
}

#[delete("/person/{id}")]
pub async fn delete_person(id: Path<String>)
                           -> Result<Json<Option<Person>>, Error> {
    let person = DB.delete(("person", &*id))
                   .await?;
    Ok(Json(person))
}
