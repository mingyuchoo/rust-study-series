# Clean Architecture TODO Application

A Rust-based TODO application built with Clean Architecture principles, featuring clear separation of concerns across domain, application, adapter, and infrastructure layers.

## Features

- RESTful API for TODO management
- **Swagger UI integration** for API documentation and testing
- SQLite database with automatic schema initialization
- Clean Architecture with dependency inversion
- Async/await support with Actix-web
- JSON serialization/deserialization
- UUID-based entity identification
- Timestamp tracking for created/updated dates
- OpenAPI 3.0 specification with detailed schemas

## Architecture

This project follows Clean Architecture principles with four distinct layers:

- **Domain Layer** (`src/domain/`): Core business entities and rules
- **Application Layer** (`src/application/`): Use cases and business logic
- **Adapter Layer** (`src/adapters/`): External interfaces (HTTP, Database)
- **Infrastructure Layer** (`src/infrastructure/`): Framework configuration and dependency injection

For detailed architecture information, see [ARCHITECTURE.md](ARCHITECTURE.md).

## API Documentation

### Swagger UI
Once the server is running, you can access the interactive API documentation at:
- **Swagger UI**: `http://localhost:8080/swagger-ui/`
- **OpenAPI JSON**: `http://localhost:8080/api-docs/openapi.json`

### API Endpoints

- `GET /api/todos` - Get all todos
- `POST /api/todos` - Create a new todo
- `PUT /api/todos/{id}` - Update an existing todo
- `DELETE /api/todos/{id}` - Delete a todo

The Swagger UI provides:
- Interactive API testing
- Request/response examples
- Schema documentation
- Authentication support

## Quick Start

### Prerequisites

- Rust (latest stable version)
- Cargo package manager

### Installation & Running

```bash
# Clone and navigate to the project
cd todo-app

# Check the project
cargo check

# Run tests
cargo test

# Run in development mode (default port 8080)
cargo run

# Run with custom port
cargo run -- --port 8000

# Build for release
cargo build --release

# Run release binary
./target/release/backend --port 8000
```

### Database

The application uses SQLite and automatically creates a `todos.db` file in the project root. The database schema is initialized automatically on startup.

### Testing the API

You can test the API using curl commands or the Swagger UI:

```bash
# Get all todos
curl -X GET http://localhost:8080/api/todos

# Create a new todo
curl -X POST http://localhost:8080/api/todos \
  -H "Content-Type: application/json" \
  -d '{"title": "Learn Rust", "description": "Study Rust programming language"}'

# Update a todo (replace {id} with actual todo ID)
curl -X PUT http://localhost:8080/api/todos/{id} \
  -H "Content-Type: application/json" \
  -d '{"title": "Learn Advanced Rust", "completed": true}'

# Delete a todo (replace {id} with actual todo ID)
curl -X DELETE http://localhost:8080/api/todos/{id}
```

## Project Structure

```
src/
├── domain/              # Domain entities and business rules
├── application/         # Use cases and repository traits  
├── adapters/           # HTTP handlers and database implementations
├── infrastructure/     # App configuration and dependency injection
├── lib.rs             # Library entry point
└── main.rs            # Application entry point
```
