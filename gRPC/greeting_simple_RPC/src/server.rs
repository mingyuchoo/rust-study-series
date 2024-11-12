use std::net::SocketAddr;
use tonic::{transport::Server,
            Request,
            Response,
            Status};

pub mod greeter_proto {
    tonic::include_proto!("communication"); // proto package
}

use greeter_proto::{greeter_server::{Greeter,
                                     GreeterServer},
                    HelloRequest,
                    HelloResponse};

#[derive(Default)]
pub struct MyGreeter {}

// Proto Service
#[tonic::async_trait]
impl Greeter for MyGreeter {
    // Proto Service rpc
    async fn say_hello(&self,
                       request: Request<HelloRequest>)
                       -> Result<Response<HelloResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let response: HelloResponse = HelloResponse { message:
                                                          format!("Hello {}!",
                                             request.into_inner()
                                                    .name), };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "[::1]:50051".parse()
                                        .unwrap();

    // Impl Proto Service
    let greeter: MyGreeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder().add_service(GreeterServer::new(greeter))
                     .serve(addr)
                     .await
                     .unwrap();

    Ok(())
}
