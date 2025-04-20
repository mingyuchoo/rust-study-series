pub mod client;
pub mod error;
pub mod server;

pub mod greeter_proto {
    tonic::include_proto!("communication"); // proto package
}
