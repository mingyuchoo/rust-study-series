use greeting_common::AppError;
use greeting_server::{parse_socket_address,
                      start_server};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let addr_str = "[::1]:50051";
    let addr = parse_socket_address(addr_str)?;
    start_server(addr).await?;
    Ok(())
}
