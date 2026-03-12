use anyhow::Result;
use tonic::transport::Channel;
use tracing::info;

pub mod proto {
    tonic::include_proto!("greeting");
}

use proto::greeting_service_client::GreetingServiceClient;
use proto::{GreetRequest, VersionRequest};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("greeting_client=info".parse()?),
        )
        .init();

    let server_addr = std::env::var("SERVER_ADDR")
        .unwrap_or_else(|_| "http://0.0.0.0:50051".into());

    info!("Connecting to gRPC server at {}", server_addr);
    let channel = Channel::from_shared(server_addr)?
        .connect()
        .await?;

    let mut client = GreetingServiceClient::new(channel);

    // Get version from WASI component via gRPC
    info!("Requesting WASM component version...");
    let version_response = client
        .get_version(VersionRequest {})
        .await?;
    println!("Version: {}", version_response.into_inner().version);

    // Send multiple greet requests
    let names = vec!["World", "WASI", "Rust", "gRPC"];
    for name in names {
        info!("Sending greet request for: {}", name);
        let response = client
            .greet(GreetRequest {
                name: name.to_string(),
            })
            .await?;
        println!("Response: {}", response.into_inner().message);
    }

    Ok(())
}
