use greeting_client::{connect_client,
                      create_and_send_request,
                      process_response};
use greeting_common::AppError;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let client = connect_client().await?;
    let response = create_and_send_request(client, "Tonic").await?;
    let message = process_response(response)?;
    println!("Successfully received message: {}", message);
    Ok(())
}
