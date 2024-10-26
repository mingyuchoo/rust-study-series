pub mod greeter_proto {
    tonic::include_proto!("communication"); // proto package
}

use greeter_proto::greeter_client::GreeterClient;
use greeter_proto::{HelloRequest, HelloResponse};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client: GreeterClient<tonic::transport::Channel> =
        GreeterClient::connect("http://[::1]:50051")
            .await
            .unwrap();

    let request: tonic::Request<HelloRequest> = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response: tonic::Response<HelloResponse> = client
        .say_hello(request)
        .await
        .unwrap();

    println!("RESPONSE={:?}", response);

    Ok(())
}
