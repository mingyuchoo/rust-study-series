# Quick Reference Guide

## Workspace Members

This workspace contains 6 crates:

1. **proto** - Shared protobuf definitions
2. **server** - gRPC server
3. **client** - gRPC client
4. **examples** - Usage examples
5. **benches** - Performance benchmarks
6. **tests** - Integration tests

## Common Commands

### Building

```bash
# Build entire workspace
cargo build

# Build specific crate
cargo build -p proto
cargo build -p server
cargo build -p client
cargo build -p examples
cargo build -p benches

# Build in release mode
cargo build --release
```

### Running

```bash
# Run server
cargo run -p server

# Run client (in another terminal)
cargo run -p client

# Run example
cargo run -p examples --bin product_service_demo
```

### Testing

```bash
# Run all tests
cargo test

# Run tests for specific crate
cargo test -p tests
cargo test -p server
```

### Benchmarking

```bash
# Run benchmarks
cargo bench -p benches

# Run specific benchmark
cargo bench -p benches --bench product_service_bench
```

### Checking

```bash
# Check entire workspace
cargo check --workspace

# Check specific crate
cargo check -p proto
```

### Cleaning

```bash
# Clean build artifacts
cargo clean
```

## Project Structure

```
ecommerce-using-grpc/
├── Cargo.toml                 # Workspace configuration
├── crates/
│   ├── proto/                 # Shared protobuf
│   ├── server/                # gRPC server
│   ├── client/                # gRPC client
│   ├── examples/              # Usage examples
│   ├── benches/               # Benchmarks
│   └── tests/                 # Integration tests
├── proto/                     # Original proto files
├── README.md                  # Main documentation
├── WORKSPACE.md               # Workspace details
├── MIGRATION_SUMMARY.md       # Migration notes
└── QUICK_REFERENCE.md         # This file
```

## Dependency Graph

```
proto (library)
  ↓
  ├─→ server (binary + library)
  ├─→ client (binary)
  ├─→ examples (binaries)
  ├─→ tests (test suite) → server
  └─→ benches (benchmarks) → server
```

## Adding Dependencies

### To a specific crate:

```bash
cd crates/server
cargo add <dependency>
```

### To workspace (shared):

Edit root `Cargo.toml`:

```toml
[workspace.dependencies]
new-dep = "1.0"
```

Then in crate's `Cargo.toml`:

```toml
[dependencies]
new-dep.workspace = true
```

## Useful Tips

- Use `-p <crate>` to target specific crates
- Use `--workspace` to target all crates
- Use `--bin <name>` for specific binaries
- Check `cargo metadata` for workspace info
- Use `cargo tree -p <crate>` to see dependencies

## Documentation

For more details, see:
- `README.md` - Project overview and usage
- `WORKSPACE.md` - Detailed workspace structure
- `MIGRATION_SUMMARY.md` - Migration from single crate
