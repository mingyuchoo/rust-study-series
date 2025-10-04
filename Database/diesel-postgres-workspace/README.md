# Diesel PostgreSQL Clean Architecture

A Rust project demonstrating Clean Architecture principles with Diesel ORM and PostgreSQL integration.

## 🚀 Quick Start

```bash
# Start everything with Docker
docker-compose -f docker/docker-compose.yaml up --build

# Open your browser
# 🌐 Web UI: http://localhost:8000
# 🔌 API: http://localhost:8000/api/todos
```

That's it! The app will automatically set up the database, run migrations, and seed sample data.

## Architecture

This project follows Clean Architecture with clear separation of concerns across multiple crates:

- **domain**: Core business entities and repository traits
- **application**: Use cases and business logic
- **adapters**: Repository implementations and external adapters  
- **infrastructure**: Database infrastructure, migrations, and models
- **main**: Application entry point and dependency injection

## Features

- Clean Architecture implementation in Rust
- PostgreSQL integration with Diesel ORM
- RESTful API with Axum web framework
- Full CRUD operations (Create, Read, Update, Delete)
- Modern web UI with vanilla JavaScript
- Docker containerization with multi-stage builds
- Database migrations and seeding
- Thread-safe database connection handling

## Prerequisites

- Rust 1.86+ (see `rust-toolchain.toml`)
- Docker and Docker Compose
- PostgreSQL (if running locally)

## Quick Start

### Using Docker (Recommended)

```bash
# Start PostgreSQL and the application
docker-compose -f docker/docker-compose.yaml up --build

# Or use make
make docker-up

# The app will automatically run migrations and create sample data
# Access the web UI at: http://localhost:8000
# Access the API at: http://localhost:8000/api/todos
```

**That's it!** Open your browser and visit `http://localhost:8000` to see the Todo app in action.

### Local Development

```bash
# Install Diesel CLI
cargo install diesel_cli --no-default-features --features postgres

# Set up environment
cp .env.test .env

# Start PostgreSQL (adjust connection details as needed)
docker run --name postgres -e POSTGRES_PASSWORD=postgres -p 5432:5432 -d postgres:17.6

# Run migrations
diesel migration run --config-file infrastructure/diesel.toml

# Run the application
cargo run -p main
```

## Project Structure

```
├── adapters/           # Repository implementations
├── application/        # Use cases (CreateTodo, ListTodos)
├── domain/            # Entities and repository traits
├── infrastructure/             # Database infrastructure
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

## API Endpoints

The REST API provides full CRUD operations:

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/todos` | List all todos |
| GET | `/api/todos/:id` | Get a specific todo |
| POST | `/api/todos` | Create a new todo |
| PUT | `/api/todos/:id` | Update a todo |
| DELETE | `/api/todos/:id` | Delete a todo |

### API Examples

```bash
# List all todos
curl http://localhost:8000/api/todos

# Create a new todo
curl -X POST http://localhost:8000/api/todos \
  -H "Content-Type: application/json" \
  -d '{"title":"Learn Rust"}'

# Update a todo
curl -X PUT http://localhost:8000/api/todos/1 \
  -H "Content-Type: application/json" \
  -d '{"title":"Master Rust"}'

# Delete a todo
curl -X DELETE http://localhost:8000/api/todos/1
```

## Web UI

The application includes a modern, responsive web interface:

- **Add todos**: Type in the input field and click "추가" or press Enter
- **Edit todos**: Click the "수정" button on any todo item
- **Delete todos**: Click the "삭제" button (with confirmation)
- **Real-time updates**: The list updates automatically after each operation

Access the web UI at `http://localhost:8000` after starting the server.

## Development

### Adding New Features

1. Define entities in `domain/src/entities.rs`
2. Add repository traits in `domain/src/repositories.rs`
3. Implement use cases in `application/src/`
4. Create repository implementations in `adapters/src/`
5. Update database schema in `infrastructure/migrations/`
6. Add API handlers in `main/src/web/handlers.rs`
7. Register routes in `main/src/web/routes.rs`

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

- `DATABASE_URL`: PostgreSQL connection string (e.g., `postgres://user:password@localhost:5432/dbname`)
- `RUST_LOG`: Logging level (default: info)

## Technology Stack

- **Web Framework**: Axum 0.7
- **ORM**: Diesel 2.3
- **Database**: PostgreSQL 17.6
- **Runtime**: Tokio (async)
- **Frontend**: Vanilla JavaScript, HTML5, CSS3

## Testing the API

Use the provided test script to verify the API endpoints:

```bash
# Make sure the server is running first
./test_api.sh
```

Or test manually with curl:

```bash
# Health check - list todos
curl http://localhost:8000/api/todos

# Create a todo
curl -X POST http://localhost:8000/api/todos \
  -H "Content-Type: application/json" \
  -d '{"title":"My new task"}'
```

## Project Architecture

This project demonstrates Clean Architecture with clear boundaries:

```
┌─────────────────────────────────────────┐
│           Presentation Layer            │
│  (Web UI + REST API Handlers)           │
│         main/src/web/                    │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│         Application Layer                │
│  (Use Cases: Create, Read, Update, etc.) │
│         application/src/                 │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│           Domain Layer                   │
│  (Entities + Repository Traits)          │
│         domain/src/                      │
└──────────────▲──────────────────────────┘
               │
┌──────────────┴──────────────────────────┐
│      Infrastructure Layer                │
│  Adapters: Repository Implementations    │
│  Infra: Database, Migrations, Models     │
│    adapters/src/ + infrastructure/src/            │
└─────────────────────────────────────────┘
```

**Benefits of this architecture:**
- Business logic is independent of frameworks
- Easy to test each layer in isolation
- Database can be swapped without changing business logic
- Clear separation of concerns

## License

This project serves as a Clean Architecture demonstration in Rust.
