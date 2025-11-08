# Cargo Workspace Structure

This project has been refactored into a Cargo workspace (monorepo) structure for better modularity and maintainability.

## Workspace Structure

```
greeting-using-grpc/
├── Cargo.toml                 # Workspace root configuration
├── proto/                     # Shared protobuf definitions
│   └── greeter.proto
└── crates/
    ├── proto/                 # Proto generation crate
    │   ├── Cargo.toml
    │   ├── build.rs
    │   └── src/
    │       └── lib.rs
    ├── common/                # Shared types and utilities
    │   ├── Cargo.toml
    │   └── src/
    │       ├── lib.rs
    │       └── error.rs
    ├── server/                # gRPC server implementation
    │   ├── Cargo.toml
    │   ├── src/
    │   │   ├── main.rs
    │   │   ├── lib.rs
    │   │   └── service.rs
    │   └── tests/
    │       └── server_service_test.rs
    └── client/                # gRPC client implementation
        ├── Cargo.toml
        ├── src/
        │   ├── main.rs
        │   ├── lib.rs
        │   └── service.rs
        ├── tests/
        │   └── integration_test.rs
        ├── benches/
        │   └── client_benchmark.rs
        └── examples/
            ├── basic_client.rs
            └── advanced_client.rs
```

## Crates

### 1. `greeting-proto`
- **Purpose**: Generates Rust code from protobuf definitions
- **Exports**: Generated gRPC service and message types
- **Dependencies**: `prost`, `tonic`, `tonic-build`

### 2. `greeting-common`
- **Purpose**: Shared error types and utilities
- **Exports**: `AppError`, `AppResult`
- **Dependencies**: `thiserror`, `tonic`

### 3. `greeting-server`
- **Purpose**: gRPC server implementation
- **Binary**: `server`
- **Dependencies**: `greeting-proto`, `greeting-common`, `tokio`, `tonic`

### 4. `greeting-client`
- **Purpose**: gRPC client implementation
- **Binary**: `client`
- **Dependencies**: `greeting-proto`, `greeting-common`, `tokio`, `tonic`

## Building

### Build entire workspace
```bash
cargo build --workspace
```

### Build specific crate
```bash
cargo build -p greeting-server
cargo build -p greeting-client
```

### Build in release mode
```bash
cargo build --workspace --release
```

## Running

### Run server
```bash
cargo run -p greeting-server --bin server
```

### Run client
```bash
cargo run -p greeting-client --bin client
```

## Testing

### Run all tests
```bash
cargo test --workspace
```

### Run tests for specific crate
```bash
cargo test -p greeting-server
cargo test -p greeting-client
```

## Examples

### Run basic client example
```bash
cargo run -p greeting-client --example basic_client
```

### Run advanced client example
```bash
cargo run -p greeting-client --example advanced_client
```

## Benchmarks

### Run client benchmarks
```bash
cargo bench -p greeting-client
```

## Benefits of Workspace Structure

1. **Modularity**: Each component is isolated in its own crate
2. **Reusability**: Common code (proto, error types) can be shared easily
3. **Independent versioning**: Each crate can be versioned independently
4. **Faster builds**: Cargo can build crates in parallel
5. **Better testing**: Tests are organized per crate
6. **Cleaner dependencies**: Each crate declares only what it needs

## Workspace Dependencies

Shared dependencies are defined in the root `Cargo.toml` under `[workspace.dependencies]`, allowing all crates to use the same versions without duplication.
