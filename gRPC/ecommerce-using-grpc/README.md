# ecommerce-using-grpc

A Rust-based gRPC service for managing product information in an e-commerce system. This project demonstrates modern Rust practices including Railway Oriented Programming, structured logging with tracing, and efficient error handling.

## Prerequisites

### Protocol Buffers Compiler

**Ubuntu**
```bash
sudo apt install protobuf-compiler
```

**Fedora**
```bash
sudo dnf install protobuf-compiler
```

**macOS**
```bash
brew install protobuf
```

## Project Structure

```
ecommerce-using-grpc/
├── proto/              # Protocol Buffer definitions
│   └── ProductInfo.proto
├── src/
│   ├── bin/
│   │   ├── server.rs   # gRPC server implementation
│   │   └── client.rs   # gRPC client implementation
│   └── lib.rs          # Service logic and error handling
├── examples/           # Example usage
├── tests/              # Integration tests
├── benches/            # Performance benchmarks
└── build.rs            # Build script for proto compilation
```

## Features

- gRPC-based product management service
- Add and retrieve product information
- Railway Oriented Programming for error handling
- Structured logging with tracing
- Comprehensive error types (NotFound, InvalidData, Internal)
- Input validation for product data

## Dependencies

- `tonic` - gRPC framework
- `prost` - Protocol Buffers implementation
- `tokio` - Async runtime
- `anyhow` - Error handling
- `thiserror` - Custom error types
- `tracing` - Structured logging

## Building the Project

```bash
# Build the project
cargo build

# Build with optimizations
cargo build --release
```

## Running the Service

### Start the Server

```bash
cargo run --bin server
```

The server will start on `[::1]:50051` (IPv6 localhost).

### Run the Client

In a separate terminal:

```bash
cargo run --bin client
```

## Running Examples

```bash
cargo run --example product_service_demo
```

## Running Tests

```bash
cargo test
```

## Running Benchmarks

```bash
cargo bench
```

## API

### AddProduct

Adds a new product to the system.

**Request:** `Product`
- `id` (int32): Product ID
- `name` (string): Product name (required, non-empty)
- `description` (string): Product description
- `price` (float): Product price (required, must be positive)

**Response:** `ProductId`
- `id` (int32): The ID of the added product

### GetProduct

Retrieves product information by ID.

**Request:** `ProductId`
- `id` (int32): Product ID (must be positive)

**Response:** `Product`
- Complete product information

## Creating a New gRPC Project

To create a similar project from scratch:

```bash
# Create new project
cargo new {project-name}
cd {project-name}

# Create directory structure
mkdir proto
touch README.md
touch build.rs

# Add dependencies
cargo add tokio --features macros,rt-multi-thread
cargo add tonic
cargo add prost
cargo add prost-types
cargo add anyhow
cargo add thiserror
cargo add tracing
cargo add tracing-subscriber

# Add build dependencies
cargo add tonic-build --build
```

## Error Handling

The service uses Railway Oriented Programming with custom error types:

- `ServiceError::NotFound` - Product not found
- `ServiceError::InvalidData` - Invalid input data
- `ServiceError::Internal` - Internal server error

All errors are properly mapped to gRPC status codes.

## License

See project license file for details.
