use greeting_common::AppError;
use greeting_server::{parse_socket_address, start_server};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let addr_str = "[::1]:50051";

    let addr = parse_socket_address(addr_str).await?;
    // Use ? to propagate the error directly
    start_server(addr).await?;
    Ok(())
}
