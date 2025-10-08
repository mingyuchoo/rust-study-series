# Tauri + Leptos

This template helps you get started developing with Tauri and Leptos.

## Project Structure

- **Frontend**: Leptos (Rust-based web framework) compiled to WebAssembly
- **Backend**: Tauri (Rust-based desktop app framework)
- **Build Tool**: Trunk for frontend bundling

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
cargo tauri dev
```

This will:
1. Run `trunk serve` to serve the Leptos frontend on http://localhost:1420
2. Launch the Tauri desktop application

## Build

Build for production:

```shell
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
