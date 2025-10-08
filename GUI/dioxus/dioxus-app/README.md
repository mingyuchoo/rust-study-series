# Dioxus App

A full-stack Dioxus application with desktop, web, and mobile support featuring document management, API integration, and local database storage.

## Features

- **Multi-platform support**: Desktop (Linux/Windows/macOS), Web (WASM), Mobile
- **Document Management**: Local SQLite database for document CRUD operations
- **API Integration**: JSONPlaceholder API for Posts, Todos, and Users
- **Modern UI**: Responsive design with tabbed interface
- **Real-time Updates**: Live data synchronization

## Prerequisites

### For Ubuntu/Debian Linux

```bash
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev \
                 build-essential \
                 pkg-config \
                 libgtk-3-dev \
                 libssl-dev \
                 libsoup-3.0-dev \
                 libxdo-dev
```

### For Fedora/RHEL Linux

```bash
sudo dnf update
sudo dnf install glib2-devel \
                 gtk3-devel \
                 webkit2gtk4.1-devel \
                 libsoup3-devel \
                 openssl-devel \
                 pkg-config \
                 libxdo-devel
```

### For macOS

```bash
# Install Xcode Command Line Tools
xcode-select --install

# Install dependencies via Homebrew
brew install pkg-config
```

### For Windows

Install Visual Studio Build Tools or Visual Studio Community with C++ development tools.

## Installation

1. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **Install Dioxus CLI**:
   ```bash
   cargo install cargo-binstall
   cargo binstall dioxus-cli
   ```

3. **Clone and setup the project**:
   ```bash
   git clone <repository-url>
   cd dioxus-app
   ```

## Key Dependencies

Based on `Cargo.toml`, this project uses:

- **Dioxus 0.6.3**: Main framework with router and fullstack features
- **Reqwest 0.12**: HTTP client for API integration with JSON support
- **Rusqlite 0.37**: SQLite database (optional, enabled with `native-db` feature)
- **Serde 1.0**: Serialization/deserialization with derive macros
- **Async-trait 0.1.89**: Async trait support
- **Dirs 6.0**: Cross-platform directory paths
- **Futures 0.3**: Async utilities

**Author**: mingyuchoo <mingyuchoo@gmail.com> (Rust Edition 2024)

## Project Structure

```
dioxus-app/
├── .cargo/
│   └── config.toml          # Linker configuration
├── assets/                  # Static assets (CSS, images, etc.)
├── src/
│   ├── application/         # Application layer (services)
│   │   ├── doc_application_service.rs
│   │   ├── post_application_service.rs
│   │   ├── todo_application_service.rs
│   │   ├── user_application_service.rs
│   │   └── mod.rs
│   ├── domain/             # Domain layer (entities, repositories)
│   ├── infrastructure/     # Infrastructure layer (API, DB)
│   ├── presentation/       # Presentation layer (UI components)
│   │   ├── docs.rs
│   │   ├── home.rs
│   │   ├── navbar.rs
│   │   ├── posts.rs
│   │   ├── todos.rs
│   │   ├── users.rs
│   │   └── mod.rs
│   └── main.rs            # Application entry point
├── Cargo.toml             # Dependencies and features
├── Dioxus.toml           # Dioxus configuration
└── README.md
```

## Development

### Running the Application

**Web (WASM) - Default:**
```bash
dx serve
# or explicitly
dx serve --platform web
```

**Desktop (recommended for full features):**
```bash
dx serve --platform desktop
```

**Mobile (requires additional setup):**
```bash
dx serve --platform mobile
```

### Building for Production

**Desktop:**
```bash
dx build --release --platform desktop
```

**Web:**
```bash
dx build --release --platform web
```

### Available Features

The application supports different feature flags in `Cargo.toml`:

- `desktop`: Desktop application with native database (includes `native-db`)
- `web`: Web application (WASM) - **default feature**
- `mobile`: Mobile application (includes `native-db`)
- `native-db`: Enables SQLite database support via `rusqlite`

### Development Commands

- **Rebuild**: Press `r` in the development server
- **Toggle auto-rebuild**: Press `p`
- **Verbose logging**: Press `v`
- **Exit**: Press `Ctrl+C`

### Web Configuration

The `Dioxus.toml` file configures web-specific settings:
- **Port**: 8080 (default development server port)
- **Output directory**: `dist/` for web builds
- **Public URL**: `/` (can be changed for deployment to subdirectories)

## Application Features

### Document Management
- Create, read, update, and delete documents
- Local SQLite database storage
- Archive/unarchive functionality
- Persistent data storage in `~/.local/share/dioxus-app/`

### API Integration
- **Posts**: Fetch and manage blog posts from JSONPlaceholder
- **Todos**: Task management with completion status
- **Users**: User information display and management

### Cross-Platform Support
- **Desktop**: Native application with full feature set
- **Web**: Browser-based application with API features
- **Mobile**: Touch-optimized interface (requires platform-specific setup)

## Troubleshooting

### Linker Issues
If you encounter linker errors on Linux, the project includes a `.cargo/config.toml` file that configures the linker to use `gcc` with `bfd` linker for x86_64-unknown-linux-gnu targets for compatibility.

### Missing Dependencies
Ensure all system dependencies are installed for your platform. The error messages will typically indicate which libraries are missing.

### Database Issues
The SQLite database is automatically created in `~/.local/share/dioxus-app/docs.db`. If you encounter database issues, you can safely delete this file to reset the database.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test`
5. Check formatting: `cargo fmt`
6. Run clippy: `cargo clippy`
7. Submit a pull request

## License

[Add your license information here]

