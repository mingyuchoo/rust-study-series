pub mod greeter_proto {
    tonic::include_proto!("communication"); // proto package
}

use greeter_proto::greeter_client::GreeterClient;
use greeter_proto::HelloRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await.unwrap();

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await.unwrap();

    println!("RESPONSE={:?}", response);

    Ok(())
}
