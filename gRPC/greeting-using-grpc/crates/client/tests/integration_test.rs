use greeting_client::{connect_client_at,
                      create_and_send_request,
                      process_response};
use greeting_common::AppResult;
use greeting_proto::greeter_proto::greeter_server::GreeterServer;
use greeting_server::MyGreeter;
use tonic::transport::Server;

#[tokio::test]
async fn test_client_server_integration() -> AppResult<()> {
    // Bind to a random available port to avoid conflicts between test runs
    let listener = tokio::net::TcpListener::bind("[::1]:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server_url = format!("http://[::1]:{}", addr.port());

    let greeter = MyGreeter::default();
    let server_task = tokio::spawn(async move {
        Server::builder()
            .add_service(GreeterServer::new(greeter))
            .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(listener))
            .await
            .unwrap();
    });

    // Connect to the test server using the dynamically assigned port
    let client = connect_client_at(&server_url).await?;

    let test_name = "Integration Test";
    let response = create_and_send_request(client, test_name).await?;
    let message = process_response(response)?;
    assert_eq!(message, format!("Hello {}!", test_name));

    server_task.abort();
    Ok(())
}
