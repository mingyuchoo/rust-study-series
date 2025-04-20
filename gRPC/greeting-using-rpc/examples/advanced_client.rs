use greeting_using_rpc::client_service::{connect_client, create_and_send_request, process_response};
use greeting_using_rpc::error::{AppError, AppResult};

#[tokio::main]
async fn main() -> AppResult<()> {
    println!("Starting advanced gRPC client example with error handling...");
    
    // Connect to the gRPC server
    let client = match connect_client().await {
        Ok(client) => {
            println!("Connected to server successfully");
            client
        },
        Err(e) => {
            eprintln!("Failed to connect to server: {}", e);
            return Err(e);
        }
    };
    
    // Try with valid name
    if let Err(e) = send_request(client.clone(), "Valid User").await {
        eprintln!("Error with valid request: {}", e);
    }
    
    // Try with empty name (should trigger an error)
    if let Err(e) = send_request(client, "").await {
        eprintln!("Expected error with empty name: {}", e);
    }
    
    Ok(())
}

async fn send_request(client: greeting_using_rpc::greeter_proto::greeter_client::GreeterClient<tonic::transport::Channel>, name: &str) -> AppResult<()> {
    println!("Sending request for name: {}", name);
    
    // Send a request
    let response = create_and_send_request(client, name).await?;
    
    // Process the response
    let message = process_response(response).await?;
    println!("Received response: {}", message);
    
    Ok(())
}
