# gRPC Greeting Service - Cargo Workspace

A modular gRPC greeting service implemented as a Cargo workspace (monorepo).

## 📁 Project Structure

This project uses a **Cargo workspace** structure with separate crates:

- **`greeting-proto`**: Protobuf definitions and generated code
- **`greeting-common`**: Shared error types and utilities
- **`greeting-server`**: gRPC server implementation
- **`greeting-client`**: gRPC client implementation

See [WORKSPACE.md](WORKSPACE.md) for detailed structure documentation.

## Prerequisites

On Ubuntu Linux:

```bash
sudo apt-get install protobuf-compiler
```

## 🚀 Quick Start

### Build

```bash
# Build entire workspace
cargo build --workspace

# Build specific crate
cargo build -p greeting-server
cargo build -p greeting-client

# Release build
cargo build --workspace --release
```

### Run Server

```bash
cargo run -p greeting-server --bin server
```

### Run Client

```bash
cargo run -p greeting-client --bin client
```

## 📚 Examples

```bash
# Run basic client example
cargo run -p greeting-client --example basic_client

# Run advanced client example with error handling
cargo run -p greeting-client --example advanced_client
```

## 🧪 Testing

```bash
# Run all tests
cargo test --workspace

# Run tests for specific crate
cargo test -p greeting-server
cargo test -p greeting-client
```

## 📊 Benchmarks

```bash
# Run all benchmarks
cargo bench --workspace

# Run client benchmarks
cargo bench -p greeting-client
```

## 📖 Documentation

- [WORKSPACE.md](WORKSPACE.md) - Detailed workspace structure and usage
- [MIGRATION.md](MIGRATION.md) - Migration guide from monolithic structure

## 🏗️ Development

This project was refactored from a monolithic crate to a workspace structure for:
- Better modularity and separation of concerns
- Parallel compilation of independent crates
- Clearer dependency management
- Improved testability
