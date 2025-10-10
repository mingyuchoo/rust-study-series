# Troubleshooting Guide

## Common Issues and Solutions

### 1. `cargo make run` fails with "unexpected argument '--manifest-path'"

**Problem**: The `cargo tauri` command doesn't accept `--manifest-path` directly.

**Solution**: The Makefile.toml has been updated to use `cwd` (current working directory) instead:

```bash
# This now works correctly
cargo make run
```

**Alternative**: Run directly from the backend directory:
```bash
cd presentation_backend
cargo tauri dev
```

### 2. Frontend not found / 404 errors

**Problem**: Dioxus frontend is not running or not accessible.

**Solution**: 
1. Make sure Dioxus CLI is installed:
   ```bash
   cargo install dioxus-cli
   ```

2. The Tauri config automatically runs `dx serve` before starting. If it fails, run manually:
   ```bash
   cd presentation_frontend
   dx serve --port 1420
   ```

3. Then in another terminal:
   ```bash
   cd presentation_backend
   cargo tauri dev
   ```

### 3. "Asset at /assets/styles.css doesn't exist"

**Problem**: Frontend assets are not in the correct location.

**Solution**: Copy assets to the frontend directory:
```bash
cp -r assets presentation_frontend/
```

Or create the assets directory:
```bash
mkdir -p presentation_frontend/assets
cp assets/styles.css presentation_frontend/assets/
```

### 4. Database errors

**Problem**: SQLite database initialization fails.

**Solution**: The app uses in-memory SQLite by default. Check the connection string in `presentation_backend/src/lib.rs`:

```rust
let database_url = "sqlite::memory:";
```

For persistent storage, change to:
```rust
let database_url = "sqlite:contacts.db";
```

### 5. Dependency resolution errors

**Problem**: Cargo can't resolve dependencies between crates.

**Solution**:
1. Clean and rebuild:
   ```bash
   cargo clean
   cargo build --workspace
   ```

2. Verify workspace structure:
   ```bash
   cargo tree -p domain --depth 1
   cargo tree -p application --depth 1
   ```

3. Check that all `Cargo.toml` files have correct paths:
   ```toml
   [dependencies]
   domain = { path = "../domain" }
   ```

### 6. "Task not found" in cargo-make

**Problem**: Running `cargo make <task>` from wrong directory.

**Solution**: Always run cargo-make commands from the workspace root:
```bash
# Wrong
cd domain
cargo make run  # ❌ Error

# Correct
cd /path/to/workspace-root
cargo make run  # ✅ Works
```

### 7. Tauri build fails

**Problem**: Missing system dependencies for Tauri.

**Solution**: Install required dependencies:

**Linux (Ubuntu/Debian)**:
```bash
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```

**macOS**:
```bash
xcode-select --install
```

**Windows**:
- Install Microsoft Visual Studio C++ Build Tools
- Install WebView2 (usually pre-installed on Windows 11)

### 8. Port already in use

**Problem**: Port 1420 is already in use.

**Solution**: 
1. Kill the process using the port:
   ```bash
   # Linux/macOS
   lsof -ti:1420 | xargs kill -9
   
   # Windows
   netstat -ano | findstr :1420
   taskkill /PID <PID> /F
   ```

2. Or change the port in `presentation_backend/tauri.conf.json`:
   ```json
   "devUrl": "http://localhost:3000"
   ```
   And update the Dioxus serve command accordingly.

### 9. Compilation errors in infrastructure layer

**Problem**: Orphan rule violations or type mismatches.

**Solution**: Use explicit error conversion instead of `From` trait:
```rust
// Instead of
impl From<sqlx::Error> for DomainError { ... }

// Use
.await
.map_err(|e| DomainError::DatabaseError(e.to_string()))?
```

### 10. Frontend and backend type mismatches

**Problem**: Contact types differ between frontend and backend.

**Solution**: 
- Backend uses `Uuid` and `DateTime<Utc>`
- Frontend uses `String` for both
- DTOs handle the conversion in `presentation_backend/src/models/contact_dto.rs`

### 11. Hot reload not working

**Problem**: Changes don't reflect in the running app.

**Solution**:
1. For frontend changes: Dioxus hot reload should work automatically
2. For backend changes: Restart `cargo tauri dev`
3. If still not working, do a clean rebuild:
   ```bash
   cargo clean
   cargo make run
   ```

### 12. Workspace member not found

**Problem**: Cargo can't find a workspace member.

**Solution**: Check the root `Cargo.toml`:
```toml
[workspace]
members = [
    "domain",
    "application",
    "infrastructure",
    "presentation_backend",
    "presentation_frontend",
]
```

All directories must exist and have their own `Cargo.toml`.

## Verification Commands

### Check workspace structure
```bash
cargo metadata --format-version 1 | jq '.workspace_members'
```

### Verify dependency rules
```bash
# Domain should have no internal dependencies
cargo tree -p domain -e normal --depth 1

# Application should only depend on domain
cargo tree -p application -e normal --depth 1

# Infrastructure should only depend on domain
cargo tree -p infrastructure -e normal --depth 1
```

### Check for circular dependencies
```bash
cargo build --workspace 2>&1 | grep -i "cyclic"
```

### Verify all crates compile
```bash
cargo check --workspace --all-targets
```

## Getting Help

If you're still experiencing issues:

1. Check the logs:
   ```bash
   cargo tauri dev 2>&1 | tee tauri.log
   ```

2. Enable verbose output:
   ```bash
   RUST_LOG=debug cargo tauri dev
   ```

3. Check Tauri info:
   ```bash
   cd presentation_backend
   cargo tauri info
   ```

4. Verify Rust toolchain:
   ```bash
   rustc --version
   cargo --version
   ```

## Clean Slate

If all else fails, start fresh:

```bash
# Remove all build artifacts
cargo clean
rm -rf target/
rm -rf presentation_backend/target/
rm -rf presentation_frontend/target/

# Remove lock files
rm Cargo.lock
rm presentation_backend/Cargo.lock
rm presentation_frontend/Cargo.lock

# Rebuild
cargo build --workspace
cargo make run
```

## Additional Resources

- [Tauri Troubleshooting](https://tauri.app/v1/guides/debugging/application)
- [Dioxus Troubleshooting](https://dioxuslabs.com/learn/0.5/getting_started)
- [Cargo Workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)
