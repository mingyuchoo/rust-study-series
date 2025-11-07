//! Simple gRPC server example
//! 
//! This example demonstrates basic usage of the greeting server.
//! 
//! Run with: cargo run --example simple_server

use greeting_common::AppError;
use greeting_server::{parse_socket_address, start_server};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    println!("=== Simple gRPC Server Example ===\n");
    
    let addr_str = "[::1]:50051";
    
    println!("Parsing socket address: {}", addr_str);
    let addr = parse_socket_address(addr_str).await?;
    println!("✓ Address parsed successfully: {}\n", addr);
    
    println!("Starting gRPC server...");
    println!("Server will listen on: {}", addr);
    println!("Press Ctrl+C to stop the server\n");
    
    // Start the server (this will block until the server is stopped)
    start_server(addr).await?;
    
    Ok(())
}
