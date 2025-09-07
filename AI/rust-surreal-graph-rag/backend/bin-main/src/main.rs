use actix_web::*;
use lib_api::run_server;
use dotenvy::dotenv;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // .env 로드(존재하지 않으면 무시)
    let _ = dotenv();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    run_server().await
}
