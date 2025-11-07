//! Simple gRPC client example
//! 
//! This example demonstrates basic usage of the greeting client.
//! 
//! Run with: cargo run --example simple_client

use greeting_client::{connect_client, create_and_send_request, process_response};
use greeting_common::AppError;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    println!("=== Simple gRPC Client Example ===\n");
    
    // Connect to the server
    println!("Connecting to server at http://[::1]:50051...");
    let client = connect_client().await?;
    println!("✓ Connected successfully!\n");
    
    // Send a greeting request
    let name = "Alice";
    println!("Sending greeting request for: {}", name);
    let response = create_and_send_request(client, name).await?;
    
    // Process the response
    let message = process_response(response).await?;
    println!("✓ Received response: {}\n", message);
    
    println!("=== Example completed successfully ===");
    Ok(())
}
