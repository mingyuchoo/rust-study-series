# Diesel PostgreSQL Clean Architecture

A Rust project demonstrating Clean Architecture principles with Diesel ORM and PostgreSQL integration.

## Architecture

This project follows Clean Architecture with clear separation of concerns across multiple crates:

- **domain**: Core business entities and repository traits
- **application**: Use cases and business logic
- **adapters**: Repository implementations and external adapters  
- **infra**: Database infrastructure, migrations, and models
- **main**: Application entry point and dependency injection

## Features

- Clean Architecture implementation in Rust
- PostgreSQL integration with Diesel ORM
- Docker containerization with multi-stage builds
- Database migrations and seeding
- Todo CRUD operations (Create, List)

## Prerequisites

- Rust 1.86+ (see `rust-toolchain.toml`)
- Docker and Docker Compose
- PostgreSQL (if running locally)

## Quick Start

### Using Docker (Recommended)

```bash
# Start PostgreSQL and the application
docker-compose -f docker/docker-compose.yaml up --build

# The app will automatically run migrations and create sample data
```

### Local Development

```bash
# Install Diesel CLI
cargo install diesel_cli --no-default-features --features postgres

# Set up environment
cp .env.test .env

# Start PostgreSQL (adjust connection details as needed)
docker run --name postgres -e POSTGRES_PASSWORD=postgres -p 5432:5432 -d postgres:17.6

# Run migrations
diesel migration run --config-file infra/diesel.toml

# Run the application
cargo run -p main
```

## Project Structure

```
├── adapters/           # Repository implementations
├── application/        # Use cases (CreateTodo, ListTodos)
├── domain/            # Entities and repository traits
├── infra/             # Database infrastructure
│   ├── migrations/    # Diesel migrations
│   └── src/          # DB models and schema
├── main/              # Application entry point
└── docker/            # Docker configuration
```

## Database Schema

The application uses a simple Todo table:

```sql
CREATE TABLE todo (
    id SERIAL PRIMARY KEY,
    title VARCHAR DEFAULT 'NEW TODO' NOT NULL
);
```

## Usage Example

The application demonstrates:

1. **Creating a Todo**: Uses `CreateTodoUseCase` to add new todos
2. **Listing Todos**: Uses `ListTodosUseCase` to retrieve all todos

```rust
// Example from main.rs
let create_uc = CreateTodoUseCase::new();
let todo = create_uc.execute(&mut repo, "Clean Architecture skeleton");

let list_uc = ListTodosUseCase::new();
let items = list_uc.execute(&mut repo);
```

## Development

### Adding New Features

1. Define entities in `domain/src/entities.rs`
2. Add repository traits in `domain/src/repositories.rs`
3. Implement use cases in `application/src/`
4. Create repository implementations in `adapters/src/`
5. Update database schema in `infra/migrations/`

### Code Formatting

The project uses custom rustfmt configuration (see `rustfmt.toml`):

```bash
cargo fmt
```

### Building

```bash
# Build all crates
cargo build

# Build specific crate
cargo build -p main
```

## Docker

The multi-stage Dockerfile optimizes build times by:
1. Caching dependencies separately from source code
2. Using slim runtime image for smaller final size
3. Including only necessary runtime dependencies

## Environment Variables

- `DATABASE_URL`: PostgreSQL connection string
- `RUST_LOG`: Logging level (default: info)

## License

This project serves as a Clean Architecture demonstration in Rust.
