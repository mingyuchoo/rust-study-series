use dotenvy::dotenv;
use std::env;

use diesel::{pg::PgConnection,
             prelude::*};

pub fn establish_connection_pg() -> PgConnection
{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).unwrap_or_else(|_| {
                                              panic!("Error connecting to {}", database_url)
                                          })
}