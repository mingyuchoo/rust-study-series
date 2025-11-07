//! Custom port gRPC server example
//! 
//! This example demonstrates running the server on a custom port.
//! 
//! Run with: cargo run --example custom_port_server

use greeting_common::AppError;
use greeting_server::{parse_socket_address, start_server};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    println!("=== Custom Port gRPC Server Example ===\n");
    
    // Use a different port (50052 instead of default 50051)
    let addr_str = "[::1]:50052";
    
    println!("Parsing socket address: {}", addr_str);
    let addr = parse_socket_address(addr_str).await?;
    println!("✓ Address parsed successfully: {}\n", addr);
    
    println!("Starting gRPC server on custom port...");
    println!("Server will listen on: {}", addr);
    println!("Note: Update client connection string to http://[::1]:50052");
    println!("Press Ctrl+C to stop the server\n");
    
    // Start the server
    start_server(addr).await?;
    
    Ok(())
}
