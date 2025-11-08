pub mod greeter_proto {
    tonic::include_proto!("communication"); // proto package
}

// Re-export commonly used types
pub use greeter_proto::*;
