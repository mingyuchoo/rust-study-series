# Cargo Workspace Structure

This document describes the Cargo workspace (monorepo) structure of the ecommerce-using-grpc project.

## Overview

The project is organized as a Cargo workspace with multiple crates, each serving a specific purpose. This structure promotes code reusability, maintainability, and clear separation of concerns.

## Workspace Members

### 1. `proto` - Protocol Buffer Definitions

**Location:** `crates/proto/`

**Purpose:** Shared protobuf definitions and generated code

**Key Files:**
- `proto/ProductInfo.proto` - Protocol buffer schema
- `build.rs` - Compiles proto files at build time
- `src/lib.rs` - Exports generated protobuf code

**Dependencies:** `prost`, `tonic`, `tonic-build`

**Used By:** All other crates (server, client, examples, tests, benches)

### 2. `server` - gRPC Server

**Location:** `crates/server/`

**Purpose:** gRPC server implementation with business logic

**Key Files:**
- `src/lib.rs` - Service implementation and error handling
- `src/main.rs` - Server binary entry point

**Dependencies:** `proto`, `tokio`, `tonic`, `anyhow`, `thiserror`, `tracing`

**Run Command:** `cargo run -p server`

### 3. `client` - gRPC Client

**Location:** `crates/client/`

**Purpose:** gRPC client implementation

**Key Files:**
- `src/main.rs` - Client binary with example usage

**Dependencies:** `proto`, `tokio`, `tonic`, `anyhow`, `tracing`

**Run Command:** `cargo run -p client`

### 4. `examples` - Usage Examples

**Location:** `crates/examples/`

**Purpose:** Demonstration code showing how to use the service

**Key Files:**
- `src/bin/product_service_demo.rs` - Example client usage

**Dependencies:** `proto`, `tokio`, `tonic`

**Run Command:** `cargo run -p examples --bin product_service_demo`

### 5. `benches` - Performance Benchmarks

**Location:** `crates/benches/`

**Purpose:** Performance benchmarking suite

**Key Files:**
- `benches/product_service_bench.rs` - Benchmark tests

**Dependencies:** `proto`, `server`, `tokio`, `tonic`

**Run Command:** `cargo bench -p benches`

### 6. `tests` - Integration Tests

**Location:** `crates/tests/`

**Purpose:** Integration testing across multiple crates

**Key Files:**
- `tests/product_service_test.rs` - Integration tests

**Dependencies:** `proto`, `server`, `tokio`, `tonic`

**Run Command:** `cargo test -p tests`

## Workspace Configuration

The root `Cargo.toml` defines:

- **Workspace members** - All crates in the workspace
- **Workspace-level dependencies** - Shared dependency versions
- **Workspace-level package metadata** - Common package information
- **Build profiles** - Shared dev and release profiles

## Dependency Graph

```
proto (shared library)
  ↓
  ├─→ server (binary + library)
  ├─→ client (binary)
  ├─→ examples (binaries)
  ├─→ tests (test suite)
  └─→ benches (benchmark suite)
      ↓
      └─→ server (for benchmarking)
```

## Benefits of This Structure

1. **Code Reusability**: The `proto` crate is shared across all other crates
2. **Clear Boundaries**: Each crate has a single, well-defined responsibility
3. **Faster Builds**: Cargo caches compiled artifacts and only rebuilds what changed
4. **Easier Testing**: Tests can depend on multiple crates without circular dependencies
5. **Better Organization**: Related code is grouped together logically
6. **Consistent Dependencies**: Workspace-level dependency management prevents version conflicts

## Common Commands

```bash
# Build entire workspace
cargo build

# Build specific crate
cargo build -p server

# Run tests for entire workspace
cargo test

# Run tests for specific crate
cargo test -p tests

# Run benchmarks
cargo bench -p benches

# Check all crates
cargo check --workspace

# Clean build artifacts
cargo clean
```

## Adding a New Crate

To add a new crate to the workspace:

1. Create the crate directory: `mkdir -p crates/new-crate`
2. Initialize the crate: `cd crates/new-crate && cargo init`
3. Add to workspace members in root `Cargo.toml`:
   ```toml
   [workspace]
   members = [
     # ... existing members
     "crates/new-crate",
   ]
   ```
4. Configure dependencies in the new crate's `Cargo.toml`

## Migration Notes

This workspace structure was created by refactoring from a single-crate structure. The original structure had:
- `src/bin/` for binaries
- `benches/` for benchmarks
- `examples/` for examples
- `tests/` for tests

These have been reorganized into separate workspace crates for better modularity and maintainability.
