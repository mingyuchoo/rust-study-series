//! Greeting logic example
//! 
//! This example demonstrates the greeting processing logic without starting a server.
//! Useful for testing the business logic independently.
//! 
//! Run with: cargo run --example greeting_logic

use greeting_common::AppError;
use greeting_server::process_greeting_request;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    println!("=== Greeting Logic Example ===\n");
    
    // Test cases
    let test_names = vec![
        "Alice",
        "Bob",
        "世界", // World in Japanese
        "Мир", // World in Russian
        "",    // Empty name (should fail)
    ];
    
    println!("Testing greeting logic with various inputs:\n");
    
    for (i, name) in test_names.iter().enumerate() {
        println!("[Test {}] Input: {:?}", i + 1, name);
        
        match process_greeting_request(name.to_string()).await {
            Ok(response) => {
                println!("  ✓ Success: {}", response.message);
            }
            Err(e) => {
                println!("  ✗ Error: {}", e);
            }
        }
        println!();
    }
    
    println!("=== Greeting logic testing completed ===");
    Ok(())
}
