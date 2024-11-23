use axum::routing::{get, post};
use axum::Router;
use clap::Parser;
use std::net::SocketAddr;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("Starting server on port {}", args.port);

    let app = Router::new().route("/", get(|| async { "Hello, World!" }))
                           .route("/echo", post(|body: String| async { body }))
                           .route("/hey", get(|| async { "Hey there!" }));

    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
    let listener = tokio::net::TcpListener::bind(&addr).await
                                                       .unwrap();
    axum::serve(listener, app).await
                              .unwrap();

    Ok(())
}
