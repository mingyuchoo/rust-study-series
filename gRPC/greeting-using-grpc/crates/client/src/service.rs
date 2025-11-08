use greeting_common::{AppError, AppResult};
use greeting_proto::greeter_proto::greeter_client::GreeterClient;
use greeting_proto::greeter_proto::{HelloRequest, HelloResponse};

pub async fn connect_client() -> AppResult<GreeterClient<tonic::transport::Channel>> {
    GreeterClient::connect("http://[::1]:50051")
        .await
        .map_err(|e| AppError::ConnectionError(format!("Failed to connect to server: {}", e)))
}

pub async fn create_and_send_request(mut client: GreeterClient<tonic::transport::Channel>, name: &str) -> AppResult<tonic::Response<HelloResponse>> {
    let request = tonic::Request::new(HelloRequest {
        name: name.into(),
    });

    client
        .say_hello(request)
        .await
        .map_err(|e| AppError::ResponseError(format!("Failed to get response: {}", e)))
}

pub async fn process_response(response: tonic::Response<HelloResponse>) -> AppResult<String> {
    let message = response.into_inner().message;
    Ok(message)
}
