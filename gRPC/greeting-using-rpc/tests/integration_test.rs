use greeting_using_rpc::client_service::{connect_client, create_and_send_request, process_response};
use greeting_using_rpc::error::AppResult;
use greeting_using_rpc::greeter_proto::greeter_server::GreeterServer;
use greeting_using_rpc::server_service::MyGreeter;
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
    let client = connect_client().await?;
    
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
