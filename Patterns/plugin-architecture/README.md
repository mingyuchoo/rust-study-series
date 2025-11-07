# Rust Plugin Architecture System

A dynamic plugin system for Rust that enables runtime feature extension through a workspace architecture. The system allows the core application to load and execute plugins without recompilation.

## Architecture

This project uses a Cargo workspace with the following structure:

```bash
plugin-architecture/
├── Cargo.toml           # Workspace root (virtual manifest)
├── Cargo.lock
├── crates/
│   ├── plugin-interface/     # Shared trait definitions for plugins
│   ├── plugin-manager/       # Plugin management library
│   └── plugins/              # Plugin implementations
│       ├── hello-plugin/     # Example: Simple greeting plugin
│       └── math-plugin/      # Example: Mathematical operations plugin
├── apps/
│   └── cli/                  # Main application that loads and manages plugins
└── target/debug/
    └── plugins/              # Runtime directory for plugin libraries
```

## Build Process

### Prerequisites

- Rust toolchain (1.70 or later recommended)
- Cargo

### Building the Project

1. **Build all workspace members:**

   ```bash
   cargo build
   ```

   This command builds:

   - `plugin-interface` crate (library)
   - `plugin-manager` executable
   - `hello-plugin` dynamic library
   - `math-plugin` dynamic library

2. **Deploy plugins to runtime directory:**

   ```bash
   ./deploy-plugins.sh
   ```

   Or on Windows:

   ```cmd
   deploy-plugins.bat
   ```

   This script:

   - Creates the `target/debug/plugins/` directory if it doesn't exist
   - Copies plugin libraries to the runtime directory
   - Handles platform-specific library extensions (.so, .dll, .dylib)

3. **Run the core application:**
   ```bash
   cargo run --bin cli
   ```

### Build Order

The workspace automatically handles build dependencies:

1. `plugin-interface` is built first (dependency for all other crates)
2. `plugin-manager` library is built next
3. Plugin implementations are built
4. `cli` application is built last

## Running the Application

After building and deploying plugins:

```bash
cargo run --bin cli
```

The application will:

1. Discover plugins in `target/debug/plugins/`
2. Load each plugin dynamically
3. Execute plugin functionality
4. Clean up and unload plugins on exit

## Creating a New Plugin

1. **Create a new crate in the plugins directory:**

   ```bash
   cd crates/plugins
   cargo new --lib my-plugin
   ```

2. **Configure Cargo.toml:**

   ```toml
   [package]
   name = "my-plugin"
   version = "0.1.0"
   edition = "2024"

   [lib]
   crate-type = ["cdylib"]

   [dependencies]
   plugin-interface = { path = "../../plugin-interface" }
   ```

3. **Implement the Plugin trait:**

   ```rust
   use plugin_interface::{Plugin, PluginContext};
   use std::error::Error;

   pub struct MyPlugin;

   impl Plugin for MyPlugin {
       fn name(&self) -> &str {
           "My Plugin"
       }

       fn version(&self) -> &str {
           "0.1.0"
       }

       fn description(&self) -> &str {
           "Description of my plugin"
       }

       fn on_load(&mut self) -> Result<(), Box<dyn Error>> {
           println!("MyPlugin loaded");
           Ok(())
       }

       fn execute(&self, context: &PluginContext) -> Result<String, Box<dyn Error>> {
           // Your plugin logic here
           Ok("Plugin executed successfully".to_string())
       }

       fn on_unload(&mut self) -> Result<(), Box<dyn Error>> {
           println!("MyPlugin unloaded");
           Ok(())
       }
   }

   #[no_mangle]
   pub extern "C" fn _plugin_create() -> *mut dyn Plugin {
       Box::into_raw(Box::new(MyPlugin))
   }
   ```

4. **Add to workspace members in root Cargo.toml:**

   ```toml
   [workspace]
   members = [
       "crates/plugin-interface",
       "crates/plugin-manager",
       "crates/plugins/hello-plugin",
       "crates/plugins/math-plugin",
       "crates/plugins/my-plugin",  # Add your plugin here
       "apps/cli",
   ]
   ```

5. **Build and deploy:**
   ```bash
   cargo build
   ./deploy-plugins.sh
   ```

## Platform-Specific Notes

### Linux

- Plugin libraries have `.so` extension
- Example: `libhello_plugin.so`

### Windows

- Plugin libraries have `.dll` extension
- Example: `hello_plugin.dll`

### macOS

- Plugin libraries have `.dylib` extension
- Example: `libhello_plugin.dylib`

## Troubleshooting

### Plugin not loading

- Ensure the plugin is built with `crate-type = ["cdylib"]`
- Verify the plugin library is in `target/debug/plugins/`
- Check that `_plugin_create` function is exported with `#[no_mangle]` and `extern "C"`

### Symbol not found errors

- Ensure plugin-interface versions match between plugin-manager and plugins
- Rebuild all workspace members: `cargo clean && cargo build`

### Runtime errors

- Check plugin implementation of the Plugin trait
- Review error messages in plugin's `on_load` or `execute` methods

## Development Workflow

1. Make changes to plugin code
2. Rebuild: `cargo build`
3. Deploy: `./deploy-plugins.sh` (or `deploy-plugins.bat` on Windows)
4. Run: `cargo run --bin cli`

## Testing

Run tests for all workspace members:

```bash
cargo test
```

Run tests for a specific crate:

```bash
cargo test -p plugin-interface
cargo test -p plugin-manager
cargo test -p hello-plugin
cargo test -p cli
```

## License

[Your License Here]
