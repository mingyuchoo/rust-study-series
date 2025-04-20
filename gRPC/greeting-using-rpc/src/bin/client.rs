use greeting_using_rpc::client_service::{connect_client, create_and_send_request, process_response};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = connect_client().await?;
    let response = create_and_send_request(client, "Tonic").await?;
    println!("RESPONSE={:?}", response);
    // Use ? to propagate the error and handle the success case
    let message = process_response(response).await?;
    println!("Successfully received message: {}", message);
    Ok(())
}
