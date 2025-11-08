# Migration Guide: Monolithic to Workspace Structure

This document explains the changes made when refactoring from a monolithic crate to a Cargo workspace.

## What Changed

### Directory Structure

**Before:**
```
greeting-using-grpc/
├── Cargo.toml
├── build.rs
├── proto/
│   └── greeter.proto
├── src/
│   ├── lib.rs
│   ├── error.rs
│   ├── mod.rs
│   └── bin/
│       ├── server.rs
│       └── client.rs
├── tests/
├── benches/
└── examples/
```

**After:**
```
greeting-using-grpc/
├── Cargo.toml (workspace root)
├── proto/
│   └── greeter.proto
└── crates/
    ├── proto/
    ├── common/
    ├── server/
    └── client/
```

### Import Path Changes

**Before:**
```rust
use greeting_using_grpc::error::{AppError, AppResult};
use greeting_using_grpc::client_service::*;
use greeting_using_grpc::server_service::*;
use greeting_using_grpc::greeter_proto::*;
```

**After:**
```rust
use greeting_common::{AppError, AppResult};
use greeting_client::*;
use greeting_server::*;
use greeting_proto::greeter_proto::*;
```

### Cargo Commands

**Before:**
```bash
# Build
cargo build

# Run server
cargo run --bin server

# Run client
cargo run --bin client

# Test
cargo test
```

**After:**
```bash
# Build
cargo build --workspace

# Run server
cargo run -p greeting-server --bin server

# Run client
cargo run -p greeting-client --bin client

# Test
cargo test --workspace
```

## Benefits

1. **Separation of Concerns**: Each component has its own crate
2. **Parallel Compilation**: Cargo can build independent crates simultaneously
3. **Clearer Dependencies**: Each crate declares only what it needs
4. **Better Testing**: Tests are organized per component
5. **Reusability**: Proto and common crates can be used independently

## Old Files

The following old files/directories can be removed after verifying the workspace works:
- `src/` (old source directory)
- `build.rs` (moved to `crates/proto/build.rs`)
- `tests/` (moved to respective crate test directories)
- `benches/` (moved to `crates/client/benches/`)
- `examples/` (moved to `crates/client/examples/`)

## Compatibility Notes

- All functionality remains the same
- Binary names remain unchanged (`server` and `client`)
- Proto definitions are unchanged
- API surface is preserved
