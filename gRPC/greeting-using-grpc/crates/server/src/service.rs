use greeting_common::{AppError, AppResult};
use greeting_proto::greeter_proto::greeter_server::{Greeter, GreeterServer};
use greeting_proto::greeter_proto::{HelloRequest, HelloResponse};
use std::net::SocketAddr;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct MyGreeter {}

// Process the request and generate a response
pub async fn process_greeting_request(name: String) -> AppResult<HelloResponse> {
    if name.is_empty() {
        return Err(AppError::RequestError("Name cannot be empty".to_string()));
    }

    Ok(HelloResponse {
        message: format!("Hello {}!", name),
    })
}

// Proto Service
#[tonic::async_trait]
impl Greeter for MyGreeter {
    // Proto Service rpc
    async fn say_hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let name = request.into_inner().name;

        match process_greeting_request(name).await {
            | Ok(response) => Ok(Response::new(response)),
            | Err(e) => {
                eprintln!("Error processing request: {}", e);
                match e {
                    | AppError::RequestError(msg) => Err(Status::invalid_argument(msg)),
                    | _ => Err(Status::internal("Internal server error")),
                }
            },
        }
    }
}

pub async fn parse_socket_address(addr_str: &str) -> AppResult<SocketAddr> {
    addr_str
        .parse()
        .map_err(|e| AppError::ServerError(format!("Failed to parse socket address: {}", e)))
}

pub async fn start_server(addr: SocketAddr) -> AppResult<()> {
    let greeter = MyGreeter::default();
    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await
        .map_err(|e| AppError::ServerError(format!("Server error: {}", e)))
}
