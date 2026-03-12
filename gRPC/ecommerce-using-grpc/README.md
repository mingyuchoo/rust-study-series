# ecommerce-using-grpc

A Rust-based gRPC service for managing product information in an e-commerce system. This project demonstrates modern Rust practices including Railway Oriented Programming, structured logging with tracing, in-memory storage, and Cargo workspace (monorepo) architecture.

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

This project uses a Cargo workspace (monorepo) structure with multiple crates:

```
ecommerce-using-grpc/
├── Cargo.toml          # Workspace configuration
├── crates/
│   ├── proto/          # Shared Protocol Buffer definitions
│   │   ├── proto/
│   │   │   └── ProductInfo.proto
│   │   ├── build.rs
│   │   └── src/lib.rs
│   ├── server/         # gRPC server implementation
│   │   └── src/
│   │       ├── lib.rs  # Service logic, in-memory store, error handling
│   │       └── main.rs # Server binary
│   ├── client/         # gRPC client implementation
│   │   └── src/
│   │       └── main.rs # Client binary
│   └── tests/          # Integration tests
│       └── tests/
│           └── product_service_test.rs
└── proto/              # Original proto files (for reference)
    └── ProductInfo.proto
```

## Features

- **Cargo Workspace (Monorepo) Architecture** - Organized into multiple focused crates
- **gRPC-based product management service** - Efficient client-server communication
- **Add and retrieve product information** - Core CRUD operations
- **In-memory storage** - Thread-safe `HashMap` with `Arc<Mutex<>>` for product persistence
- **Auto-incremented IDs** - Server assigns product IDs via atomic counter
- **Railway Oriented Programming** - Clean error handling pattern
- **Structured logging with tracing** - Production-ready observability
- **Comprehensive error types** - NotFound, InvalidData, Internal
- **Input validation** - Product name (non-empty) and price (positive) validation
- **Shared proto definitions** - Reusable protobuf package

## Dependencies

| Crate | Version | Purpose |
|---|---|---|
| `tonic` | 0.14.5 | gRPC framework |
| `prost` | 0.14.3 | Protocol Buffers implementation |
| `tokio` | 1.50.0 | Async runtime |
| `anyhow` | 1.0.102 | Error handling |
| `thiserror` | 2.0.18 | Custom error types |
| `tracing` | 0.1.44 | Structured logging |
| `tracing-subscriber` | 0.3.22 | Log formatting and output |

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
cargo run -p server
```

The server will start on `[::1]:50051` (IPv6 localhost).

### Run the Client

In a separate terminal:

```bash
cargo run -p client
```

## Running Tests

```bash
# Run all tests in the workspace
cargo test

# Run tests for a specific crate
cargo test -p tests
```

## API

### AddProduct

Adds a new product to the system. The server automatically assigns a unique ID.

**Request:** `Product`
- `id` (int32): Ignored — the server assigns a new auto-incremented ID
- `name` (string): Product name (required, non-empty)
- `description` (string): Product description
- `price` (float): Product price (required, must be positive)

**Response:** `ProductId`
- `id` (int32): The server-assigned ID of the added product

### GetProduct

Retrieves product information by ID.

**Request:** `ProductId`
- `id` (int32): Product ID (must be positive; returns `NOT_FOUND` if no product exists with that ID)

**Response:** `Product`
- Complete product information as stored

## Error Handling

The service uses Railway Oriented Programming with custom error types:

| Error | gRPC Status | Condition |
|---|---|---|
| `ServiceError::NotFound` | `NOT_FOUND` | No product exists for the given ID |
| `ServiceError::InvalidData` | `INVALID_ARGUMENT` | Empty name or non-positive price |
| `ServiceError::Internal` | `INTERNAL` | Unexpected server-side failure |

## Workspace Benefits

This monorepo structure provides several advantages:

- **Code Reusability** - The `proto` crate is shared across all other crates
- **Consistent Dependencies** - Workspace-level dependency management ensures version consistency
- **Faster Builds** - Cargo can cache and reuse compiled artifacts across crates
- **Better Organization** - Clear separation of concerns (server, client, tests)
- **Easier Testing** - Integration tests can easily depend on multiple crates
- **Simplified CI/CD** - Single repository for all related code

## Creating a Similar Workspace

To create a similar workspace from scratch:

```bash
# Create workspace root
mkdir {project-name}
cd {project-name}

# Create workspace Cargo.toml
cat > Cargo.toml << 'EOF'
[workspace]
resolver = "2"
members = [
  "crates/proto",
  "crates/server",
  "crates/client",
]

[workspace.package]
edition = "2024"
version = "0.1.0"

[workspace.dependencies]
tokio = { version = "1.50", features = ["macros", "rt-multi-thread"] }
tonic = "0.14"
prost = "0.14"
# ... other dependencies
EOF

# Create crate structure
mkdir -p crates/{proto,server,client}

# Initialize each crate
cd crates/proto && cargo init --lib && cd ../..
cd crates/server && cargo init && cd ../..
cd crates/client && cargo init && cd ../..
```

## License

See project license file for details.
