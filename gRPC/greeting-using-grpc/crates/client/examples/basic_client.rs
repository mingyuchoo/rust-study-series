use greeting_client::{connect_client, create_and_send_request, process_response};
use greeting_common::AppResult;

#[tokio::main]
async fn main() -> AppResult<()> {
    println!("Starting basic gRPC client example...");
    
    // Connect to the gRPC server
    let client = connect_client().await?;
    println!("Connected to server successfully");
    
    // Send a request with a custom name
    let name = "Example User";
    let response = create_and_send_request(client, name).await?;
    println!("Sent request for name: {}", name);
    
    // Process the response
    let message = process_response(response).await?;
    println!("Received response: {}", message);
    
    Ok(())
}
