//! Multiple requests gRPC client example
//! 
//! This example demonstrates sending multiple requests to the server.
//! 
//! Run with: cargo run --example multiple_requests

use greeting_client::{connect_client, create_and_send_request, process_response};
use greeting_common::AppError;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    println!("=== Multiple Requests gRPC Client Example ===\n");
    
    // Connect to the server
    println!("Connecting to server at http://[::1]:50051...");
    let client = connect_client().await?;
    println!("✓ Connected successfully!\n");
    
    // List of names to greet
    let names = vec!["Alice", "Bob", "Charlie", "David", "Eve"];
    
    println!("Sending {} greeting requests...\n", names.len());
    
    // Send multiple requests
    for (i, name) in names.iter().enumerate() {
        println!("[{}] Greeting: {}", i + 1, name);
        let response = create_and_send_request(client.clone(), name).await?;
        let message = process_response(response).await?;
        println!("    Response: {}\n", message);
    }
    
    println!("=== All {} requests completed successfully ===", names.len());
    Ok(())
}
