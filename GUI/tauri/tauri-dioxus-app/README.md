# Tauri + Dioxus App

A cross-platform desktop application built with Tauri and Dioxus, combining the power of Rust for both frontend and backend development.

## Project Structure

```
tauri-dioxus-app/
├── src/                    # Dioxus frontend source
│   ├── main.rs            # Frontend entry point
│   └── app.rs             # Main app component
├── src-tauri/             # Tauri backend source
│   ├── src/
│   │   ├── main.rs        # Backend entry point
│   │   └── lib.rs         # Tauri commands and setup
│   ├── Cargo.toml         # Backend dependencies
│   └── tauri.conf.json    # Tauri configuration
├── assets/                # Static assets (CSS, images)
├── Cargo.toml            # Frontend dependencies
├── Dioxus.toml           # Dioxus configuration
└── Makefile.toml         # Build tasks
```

## Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [Dioxus CLI](https://dioxuslabs.com/learn/0.6/getting_started): `cargo install dioxus-cli`
- [Tauri CLI](https://tauri.app/start/prerequisites/): `cargo install tauri-cli`

## Development

### Quick Start

```shell
# Clone and navigate to project
cd tauri-dioxus-app

# Install dependencies (handled by cargo)
# Run development server
cargo tauri dev
```

### Available Commands

Using cargo-make (install with `cargo install cargo-make`):

```shell
# Development
cargo make run              # Start development server
cargo make watch-run        # Start with file watching

# Code Quality
cargo make check            # Check code
cargo make clippy           # Run linter
cargo make format           # Format code
cargo make test             # Run tests

# Build
cargo make build            # Development build
cargo make release          # Production build
cargo make clean            # Clean build artifacts
```

### Direct Commands

```shell
# Development
cargo tauri dev             # Start development server
dx serve --port 1420        # Start Dioxus dev server only

# Build
cargo tauri build           # Build for production
dx bundle --release         # Build Dioxus bundle only
```

## Features

- **Cross-platform**: Runs on Windows, macOS, and Linux
- **Rust Frontend**: Dioxus for reactive UI components
- **Rust Backend**: Tauri for native system access
- **Hot Reload**: Fast development with automatic reloading
- **Asset Management**: Integrated asset handling
- **Type Safety**: Full Rust type safety across the stack

## Configuration

- **Dioxus Config**: `Dioxus.toml` - Frontend build settings
- **Tauri Config**: `src-tauri/tauri.conf.json` - App metadata and build settings
- **Build Tasks**: `Makefile.toml` - Development workflow automation

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) with extensions:
- [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- [Dioxus](https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus)

## Learn More

- [Tauri Documentation](https://tauri.app/)
- [Dioxus Documentation](https://dioxuslabs.com/)
- [Rust Book](https://doc.rust-lang.org/book/)
