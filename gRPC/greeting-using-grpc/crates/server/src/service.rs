use greeting_common::{AppError,
                      AppResult};
use greeting_proto::greeter_proto::greeter_server::{Greeter,
                                                    GreeterServer};
use greeting_proto::greeter_proto::{HelloRequest,
                                    HelloResponse};
use std::net::SocketAddr;
use tonic::transport::Server;
use tonic::{Request,
            Response,
            Status};
use tracing::{error,
              info};

#[derive(Default)]
pub struct MyGreeter {}

// Process the request and generate a response (sync — no I/O involved)
pub fn process_greeting_request(name: String) -> AppResult<HelloResponse> {
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
    async fn say_hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloResponse>, Status> {
        info!(remote_addr = ?request.remote_addr(), "Received request");

        let name = request.into_inner().name;

        match process_greeting_request(name) {
            | Ok(response) => Ok(Response::new(response)),
            | Err(e) => {
                error!(error = %e, "Error processing request");
                match e {
                    | AppError::RequestError(msg) => Err(Status::invalid_argument(msg)),
                    | _ => Err(Status::internal("Internal server error")),
                }
            },
        }
    }
}

// Parses a socket address string (sync — pure string parsing)
pub fn parse_socket_address(addr_str: &str) -> AppResult<SocketAddr> {
    addr_str
        .parse()
        .map_err(|e| AppError::ServerError(format!("Failed to parse socket address: {}", e)))
}

pub async fn start_server(addr: SocketAddr) -> AppResult<()> {
    let greeter = MyGreeter::default();
    info!(%addr, "GreeterServer listening");

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await
        .map_err(|e| AppError::ServerError(format!("Server error: {}", e)))
}
