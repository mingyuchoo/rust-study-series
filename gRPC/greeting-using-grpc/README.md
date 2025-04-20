# README

## Prerequisites

On Ubuntu Linux

```bash
sudo apt-get install protobuf-compiler
```

## Working Process

```bash
cargo new {project-name}
cd {project-name}
touch README.md
mkdir proto data
cargo add tokio --features tokio/macros,rt-multi-thread
cargo add tonic-build --build
cargo add tonic
cargo add prost
cargo add prost-types
touch build.rs
```

## How to Run

### Run Server

```bash
cargo run --bin server
```

### Run Client

```bash
cargo run --bin client
```

### Run Examples

```bash
# Run basic client example
cargo run --example basic_client

# Run advanced client example with error handling
cargo run --example advanced_client
```

### Run Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test --test server_service_test
cargo test --test integration_test
```

### Run Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench client_benchmark
```
