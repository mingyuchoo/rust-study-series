//! Concurrent requests gRPC client example
//! 
//! This example demonstrates sending multiple concurrent requests to the server.
//! 
//! Run with: cargo run --example concurrent_requests

use greeting_client::{connect_client, create_and_send_request, process_response};
use greeting_common::AppError;
use tokio::task::JoinSet;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    println!("=== Concurrent Requests gRPC Client Example ===\n");
    
    // Connect to the server
    println!("Connecting to server at http://[::1]:50051...");
    let client = connect_client().await?;
    println!("✓ Connected successfully!\n");
    
    // List of names to greet concurrently
    let names = vec!["Alice", "Bob", "Charlie", "David", "Eve", "Frank", "Grace", "Henry"];
    
    println!("Sending {} concurrent greeting requests...\n", names.len());
    
    // Create a JoinSet to manage concurrent tasks
    let mut join_set = JoinSet::new();
    
    // Spawn concurrent requests
    for name in names {
        let client_clone = client.clone();
        join_set.spawn(async move {
            let response = create_and_send_request(client_clone, name).await?;
            let message = process_response(response).await?;
            Ok::<(String, String), AppError>((name.to_string(), message))
        });
    }
    
    // Collect results
    let mut results = Vec::new();
    while let Some(result) = join_set.join_next().await {
        match result {
            Ok(Ok((name, message))) => {
                results.push((name, message));
            }
            Ok(Err(e)) => {
                eprintln!("Request failed: {}", e);
            }
            Err(e) => {
                eprintln!("Task join failed: {}", e);
            }
        }
    }
    
    // Display results
    println!("Received {} responses:\n", results.len());
    for (i, (name, message)) in results.iter().enumerate() {
        println!("[{}] {} -> {}", i + 1, name, message);
    }
    
    println!("\n=== All concurrent requests completed ===");
    Ok(())
}
