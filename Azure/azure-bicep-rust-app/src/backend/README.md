# Clean Architecture TODO Application

A Rust-based TODO application built with Clean Architecture principles, featuring clear separation of concerns across domain, application, adapter, and infrastructure layers.

## Features

- RESTful API for TODO management
- SQLite database with automatic schema initialization
- Clean Architecture with dependency inversion
- Async/await support with Actix-web
- JSON serialization/deserialization
- UUID-based entity identification
- Timestamp tracking for created/updated dates

## Architecture

This project follows Clean Architecture principles with four distinct layers:

- **Domain Layer** (`src/domain/`): Core business entities and rules
- **Application Layer** (`src/application/`): Use cases and business logic
- **Adapter Layer** (`src/adapters/`): External interfaces (HTTP, Database)
- **Infrastructure Layer** (`src/infrastructure/`): Framework configuration and dependency injection

For detailed architecture information, see [ARCHITECTURE.md](ARCHITECTURE.md).

## API Endpoints

- `GET /api/todos` - Get all todos
- `POST /api/todos` - Create a new todo
- `PUT /api/todos/{id}` - Update an existing todo
- `DELETE /api/todos/{id}` - Delete a todo

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
