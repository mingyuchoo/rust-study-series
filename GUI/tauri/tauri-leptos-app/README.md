# Tauri + Leptos with Clean Architecture

This project demonstrates a Clean Architecture implementation using Tauri and Leptos, with each layer separated into its own crate to enforce dependency rules.

## Clean Architecture Structure

The project follows Clean Architecture principles with strict dependency rules enforced through Cargo workspace separation:

```
project-root/
├── Cargo.toml              # Workspace configuration
├── domain/                 # 1. Domain Layer (innermost - no dependencies)
│   ├── src/
│   │   ├── entities.rs     # Core business entities
│   │   ├── errors.rs       # Domain errors
│   │   └── repositories.rs # Repository interfaces (traits)
│   └── Cargo.toml
├── application/            # 2. Application Layer (use cases)
│   ├── src/usecases/       # Business logic implementation
│   └── Cargo.toml          # Depends only on `domain`
├── infrastructure/         # 3. Infrastructure Layer (external concerns)
│   ├── src/database/       # Database implementations
│   └── Cargo.toml          # Depends on `domain` and `application`
├── presentation-frontend/  # 4. Frontend Presentation Layer
│   ├── src/                # Leptos UI components and API clients
│   └── Cargo.toml          # Depends on `domain` only
└── presentation-backend/   # 4. Backend Presentation Layer
    ├── src/                # Tauri command handlers
    └── Cargo.toml          # Depends on all layers
```

### Dependency Rules

- **Domain**: No dependencies (pure business logic)
- **Application**: Depends only on Domain
- **Infrastructure**: Depends on Domain and Application
- **Presentation**: Can depend on Domain, Application, and Infrastructure as needed

This structure ensures that:
- Business logic is independent of frameworks and external concerns
- Dependencies point inward (toward the domain)
- Each layer can be tested in isolation
- Changes in outer layers don't affect inner layers

## Prerequisites

Install the required tools:

```shell
cargo install create-tauri-app --locked
cargo install tauri-cli --locked
cargo install trunk --locked
```

## Development

Start the development server:

```shell
cd presentation-backend
cargo tauri dev
```

This will:
1. Run `trunk serve` to serve the Leptos frontend on http://localhost:1420
2. Launch the Tauri desktop application

The frontend code is in `presentation-frontend/` and the backend code is in `presentation-backend/`.

## Build

Build for production:

```shell
cd presentation-backend
cargo tauri build
```

## Available Tasks (using cargo-make)

```shell
# Install cargo-make first
cargo install cargo-make

# Available tasks
cargo make clean      # Clean build artifacts
cargo make check      # Check code
cargo make clippy     # Run clippy linter
cargo make format     # Format code
cargo make build      # Build in dev mode
cargo make release    # Build in release mode
cargo make test       # Run tests
cargo make run        # Run tauri dev
cargo make watch-run  # Watch and auto-restart (requires cargo-watch)
```

## Project Details

- **Project Name**: tauri-leptos-app
- **Identifier**: com.mingyuchoo.tauri-leptos-app
- **Frontend Framework**: Leptos 0.7 with CSR (Client-Side Rendering)
- **Window Size**: 800x600

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
