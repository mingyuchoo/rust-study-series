use greeting_client::{create_and_send_request, process_response};
use greeting_common::{AppError, AppResult};
use greeting_proto::greeter_proto::greeter_client::GreeterClient;
use greeting_proto::greeter_proto::greeter_server::GreeterServer;
use greeting_server::MyGreeter;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::time::sleep;
use tonic::transport::Server;

#[tokio::test]
async fn test_client_server_integration() -> AppResult<()> {
    // Start a test server in the background
    let addr: SocketAddr = "[::1]:50052".parse().unwrap();
    let greeter = MyGreeter::default();
    
    let server_task = tokio::spawn(async move {
        Server::builder()
            .add_service(GreeterServer::new(greeter))
            .serve(addr)
            .await
            .unwrap();
    });
    
    // Give the server a moment to start
    sleep(Duration::from_millis(100)).await;
    
    // Connect to the test server
    let client = GreeterClient::connect("http://[::1]:50052")
        .await
        .map_err(|e| AppError::ConnectionError(format!("Failed to connect to server: {}", e)))?;
    
    // Send a request
    let test_name = "Integration Test";
    let response = create_and_send_request(client, test_name).await?;
    
    // Process and verify the response
    let message = process_response(response).await?;
    assert_eq!(message, format!("Hello {}!", test_name));
    
    // Clean up
    server_task.abort();
    
    Ok(())
}
