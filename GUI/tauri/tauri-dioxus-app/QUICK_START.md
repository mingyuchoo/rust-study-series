# Quick Start Guide

## 🚀 Common Commands

### Development

```bash
# Run the application in development mode
cargo make run
# or
cargo make dev

# Alternative: Run directly from presentation_backend
cd presentation_backend
cargo tauri dev

# Build the workspace
cargo make build

# Check code without building
cargo make check

# Format code
cargo make format

# Run linter
cargo make clippy
```

### Testing

```bash
# Run all tests
cargo make test

# Test specific layers
cargo make test-domain
cargo make test-application
cargo make test-infrastructure

# Test specific crate
cargo test -p domain
cargo test -p application
```

### Production

```bash
# Build for production
cargo make build-app

# Or directly with cargo
cargo tauri build --manifest-path presentation_backend/Cargo.toml
```

### Verification

```bash
# Check entire workspace
cargo check --workspace

# Verify dependency rules
cargo tree -p domain --depth 1        # Should have no internal deps
cargo tree -p application --depth 1   # Should only depend on domain
cargo tree -p infrastructure --depth 1 # Should only depend on domain
```

## 📁 Project Structure

```
├── domain/                 # Core business logic (no dependencies)
├── application/            # Use cases (depends on domain)
├── infrastructure/         # Database implementation (depends on domain)
├── presentation_backend/   # Tauri backend (depends on all)
└── presentation_frontend/  # Dioxus frontend (independent)
```

## 🔧 Troubleshooting

### Build Errors

```bash
# Clean and rebuild
cargo clean
cargo build --workspace
```

### Dependency Issues

```bash
# Update dependencies
cargo update

# Check for outdated dependencies
cargo outdated
```

### Tauri Issues

```bash
# Check Tauri CLI version
cargo tauri --version

# Reinstall Tauri CLI if needed
cargo install tauri-cli --version "^2.0.0"
```

## 📚 Documentation

- **README_FINAL.md** - Complete project documentation
- **MIGRATION_GUIDE.md** - How to migrate to this structure
- **CLEAN_ARCHITECTURE_SUMMARY.md** - Architecture overview
- **DEPENDENCY_DIAGRAM.md** - Visual dependency diagrams

## 🎯 Next Steps

1. **Add Tests**: Create unit tests in each layer
2. **Add Features**: Implement new use cases in `application/`
3. **Extend Infrastructure**: Add PostgreSQL support in `infrastructure/`
4. **Add UI**: Create new components in `presentation_frontend/`

## 💡 Tips

- Always run `cargo make check` before committing
- Use `cargo make format` to auto-format code
- Test each layer independently with `cargo test -p <crate-name>`
- Verify dependency rules with `cargo tree`

## 🐛 Common Issues

### "Task not found" error

Make sure you're running commands from the workspace root, not from individual crate directories.

### Tauri config not found

The Tauri config is in `presentation_backend/tauri.conf.json`. Always use:
```bash
cargo tauri dev --manifest-path presentation_backend/Cargo.toml
```

### Frontend assets not found

Assets should be in `presentation_frontend/assets/`. Make sure to copy them:
```bash
cp -r assets presentation_frontend/
```

## 🎓 Learning Resources

- [Clean Architecture](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)
- [Cargo Workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)
- [Tauri Docs](https://tauri.app/)
- [Dioxus Docs](https://dioxuslabs.com/)
