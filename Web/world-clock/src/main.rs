mod cli;
mod clock;
mod config;
mod error;
mod web;

use std::path::PathBuf;
use std::sync::Arc;

use chrono::Utc;
use clap::Parser;

use cli::{Cli, Commands};
use clock::{format_clocks, get_clock_display};
use config::{CityEntry, Config, default_config_path};
use error::AppError;
use web::AppState;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("에러: {e}");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), AppError> {
    let cli = Cli::parse();
    let config_path = cli.config.unwrap_or_else(default_config_path);

    match cli.command {
        Some(Commands::Add { name, timezone }) => {
            let mut config = Config::load(&config_path)?;
            config.add(CityEntry { name, timezone })?;
            config.save(&config_path)?;
            println!("도시가 추가되었습니다.");
        }
        Some(Commands::Remove { name }) => {
            let mut config = Config::load(&config_path)?;
            config.remove(&name)?;
            config.save(&config_path)?;
            println!("도시가 삭제되었습니다.");
        }
        Some(Commands::List) => {
            let config = Config::load(&config_path)?;
            if config.cities.is_empty() {
                println!("저장된 도시가 없습니다.");
            } else {
                for city in &config.cities {
                    println!("{} ({})", city.name, city.timezone);
                }
            }
        }
        Some(Commands::Serve { port }) => {
            let config = Config::load(&config_path)?;
            let registry_path = config_path
                .parent()
                .unwrap_or_else(|| std::path::Path::new("."))
                .join("../docs/registry.json");
            let registry_path = std::fs::canonicalize(&registry_path)
                .unwrap_or_else(|_| PathBuf::from("docs/registry.json"));
            let state = Arc::new(AppState::new(config, config_path, registry_path));
            let app = web::create_router(state);

            let addr = format!("0.0.0.0:{port}");
            println!("웹 서버 시작: http://localhost:{port}");

            let listener = tokio::net::TcpListener::bind(&addr)
                .await
                .map_err(AppError::Config)?;
            axum::serve(listener, app).await.map_err(AppError::Config)?;
        }
        None => {
            let config = Config::load(&config_path)?;
            let now = Utc::now();
            let displays = config
                .cities
                .iter()
                .map(|c| get_clock_display(&c.name, &c.timezone, now))
                .collect::<Result<Vec<_>, _>>()?;
            print!("{}", format_clocks(&displays));
        }
    }

    Ok(())
}
