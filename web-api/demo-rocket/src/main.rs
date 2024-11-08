mod controllers;
mod database;
pub mod routes;
mod schema;
mod services;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    routes::build().launch()
                   .await?;

    Ok(())
}
