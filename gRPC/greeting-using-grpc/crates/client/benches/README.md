# Client Benchmarks

## Prerequisites

Before running benchmarks, ensure the gRPC server is running:

```bash
# In one terminal, start the server
cargo run -p greeting-server --bin server
```

## Running Benchmarks

```bash
# In another terminal, run the benchmarks
cargo bench -p greeting-client
```

## Benchmark Results

Results will be saved to `target/criterion/` directory.

## Note

The benchmarks require a running server on `[::1]:50051`. If the server is not running, the benchmarks will fail with a connection error.
