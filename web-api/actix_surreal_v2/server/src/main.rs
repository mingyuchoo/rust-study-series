use actix_web::*;
use actix_lib::run_server;
//use env_logger;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    run_server().await
}
