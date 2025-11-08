# Architecture Overview

## Dependency Graph

```
┌─────────────────────────────────────────────────────────────┐
│                     Workspace Root                          │
│                   (Cargo.toml)                              │
└─────────────────────────────────────────────────────────────┘
                              │
                ┌─────────────┴─────────────┐
                │                           │
                ▼                           ▼
    ┌───────────────────┐       ┌───────────────────┐
    │  greeting-server  │       │  greeting-client  │
    │   (Binary Crate)  │       │   (Binary Crate)  │
    └───────────────────┘       └───────────────────┘
                │                           │
                └─────────────┬─────────────┘
                              │
                ┌─────────────┴─────────────┐
                │                           │
                ▼                           ▼
    ┌───────────────────┐       ┌───────────────────┐
    │  greeting-proto   │       │  greeting-common  │
    │  (Library Crate)  │       │  (Library Crate)  │
    └───────────────────┘       └───────────────────┘
                │
                ▼
    ┌───────────────────┐
    │   proto/*.proto   │
    │  (Proto Files)    │
    └───────────────────┘
```

## Crate Responsibilities

### 1. `greeting-proto`
**Type**: Library  
**Purpose**: Protocol Buffer definitions and code generation

**Responsibilities**:
- Compile `.proto` files to Rust code
- Export gRPC service definitions
- Export message types (HelloRequest, HelloResponse)
- Provide client and server stubs

**Dependencies**:
- `prost` - Protocol Buffer implementation
- `tonic` - gRPC framework
- `tonic-build` - Build-time proto compilation

**Exports**:
```rust
pub mod greeter_proto {
    // Generated code
    pub mod greeter_client { ... }
    pub mod greeter_server { ... }
    pub struct HelloRequest { ... }
    pub struct HelloResponse { ... }
}
```

---

### 2. `greeting-common`
**Type**: Library  
**Purpose**: Shared types and utilities

**Responsibilities**:
- Define application error types
- Provide error conversion traits
- Export common type aliases

**Dependencies**:
- `thiserror` - Error derive macros
- `tonic` - For Status conversion

**Exports**:
```rust
pub enum AppError {
    ConnectionError(String),
    RequestError(String),
    ResponseError(String),
    ServerError(String),
}

pub type AppResult<T> = Result<T, AppError>;
```

---

### 3. `greeting-server`
**Type**: Binary + Library  
**Purpose**: gRPC server implementation

**Responsibilities**:
- Implement Greeter service trait
- Handle incoming gRPC requests
- Process greeting logic
- Manage server lifecycle

**Dependencies**:
- `greeting-proto` - Service definitions
- `greeting-common` - Error types
- `tokio` - Async runtime
- `tonic` - gRPC server

**Binary**: `server`
**Exports**:
```rust
pub struct MyGreeter { ... }
pub async fn process_greeting_request(...) -> AppResult<HelloResponse>
pub async fn parse_socket_address(...) -> AppResult<SocketAddr>
pub async fn start_server(...) -> AppResult<()>
```

---

### 4. `greeting-client`
**Type**: Binary + Library  
**Purpose**: gRPC client implementation

**Responsibilities**:
- Connect to gRPC server
- Send greeting requests
- Process responses
- Provide client utilities

**Dependencies**:
- `greeting-proto` - Service definitions
- `greeting-common` - Error types
- `tokio` - Async runtime
- `tonic` - gRPC client

**Binary**: `client`
**Exports**:
```rust
pub async fn connect_client() -> AppResult<GreeterClient<...>>
pub async fn create_and_send_request(...) -> AppResult<Response<HelloResponse>>
pub async fn process_response(...) -> AppResult<String>
```

**Dev Dependencies**:
- `greeting-server` - For integration tests
- `criterion` - For benchmarks

---

## Data Flow

### Server Flow
```
1. main.rs
   ↓
2. parse_socket_address()
   ↓
3. start_server()
   ↓
4. MyGreeter::say_hello() ← [gRPC Request]
   ↓
5. process_greeting_request()
   ↓
6. [gRPC Response] →
```

### Client Flow
```
1. main.rs
   ↓
2. connect_client()
   ↓
3. create_and_send_request()
   ↓
4. [gRPC Request] →
   ↓
5. ← [gRPC Response]
   ↓
6. process_response()
   ↓
7. Display result
```

## Build Process

### Proto Generation (greeting-proto)
```
build.rs
   ↓
tonic-prost-build::compile_protos()
   ↓
Read: proto/greeter.proto
   ↓
Generate: target/OUT_DIR/communication.rs
   ↓
Include via: tonic::include_proto!("communication")
```

### Workspace Build Order
```
1. greeting-proto   (no dependencies)
2. greeting-common  (no dependencies)
3. greeting-server  (depends on proto, common)
4. greeting-client  (depends on proto, common)
```

## Testing Strategy

### Unit Tests
- **Server**: `crates/server/tests/server_service_test.rs`
  - Test greeting request processing
  - Test empty name validation

### Integration Tests
- **Client**: `crates/client/tests/integration_test.rs`
  - Test full client-server interaction
  - Spawn test server, connect, send request, verify response

### Benchmarks
- **Client**: `crates/client/benches/client_benchmark.rs`
  - Measure gRPC request/response performance

## Module Organization

### greeting-server
```
src/
├── main.rs       → Binary entry point
├── lib.rs        → Library exports
└── service.rs    → Service implementation
    ├── MyGreeter struct
    ├── Greeter trait impl
    ├── process_greeting_request()
    ├── parse_socket_address()
    └── start_server()
```

### greeting-client
```
src/
├── main.rs       → Binary entry point
├── lib.rs        → Library exports
└── service.rs    → Client utilities
    ├── connect_client()
    ├── create_and_send_request()
    └── process_response()
```

## Configuration Management

### Workspace-level
- Shared dependency versions
- Common profile settings (dev, release)
- Workspace metadata (authors, edition)

### Crate-level
- Crate-specific dependencies
- Binary configurations
- Dev dependencies (tests, benchmarks)

## Future Extensibility

The workspace structure allows for easy addition of:
- New service implementations
- Additional proto definitions
- Shared middleware crates
- Common utility libraries
- Multiple client/server variants
