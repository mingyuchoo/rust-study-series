mod schema;
mod database;
mod services;
mod controllers;
pub mod routes;


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    routes::build()
        .launch().await?;

    Ok(())
}

