/*
crate(main)
  |- schema
  |- database
  |- models
  |- controllers
        |- posts
*/
mod controllers;
mod database;
mod models;
mod routes;
mod schema;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    routes::build().launch().await?;

    Ok(())
}

