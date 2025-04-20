pub mod error;

pub mod greeter_proto {
    tonic::include_proto!("communication"); // proto package
}

pub mod client_service {
    use crate::error::{AppError, AppResult};
    use crate::greeter_proto::greeter_client::GreeterClient;
    use crate::greeter_proto::{HelloRequest, HelloResponse};

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
}

pub mod server_service {
    use crate::error::{AppError, AppResult};
    use crate::greeter_proto::greeter_server::{Greeter, GreeterServer};
    use crate::greeter_proto::{HelloRequest, HelloResponse};
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
}
