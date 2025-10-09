# Migration Guide: From Monolithic to Clean Architecture

This guide helps you migrate from the original monolithic structure to the new Clean Architecture with separate crates.

## What Changed

### Old Structure
```
src/
├── app.rs
├── main.rs
├── domain/
├── services/
└── use_cases/
```

### New Structure
```
domain/                 # Pure business logic
application/            # Use cases
infrastructure/         # Database, external services
presentation-frontend/  # Leptos UI
presentation-backend/   # Tauri handlers
```

## Migration Steps

### 1. Update Dependencies

The new structure uses a workspace. Your root `Cargo.toml` now defines shared dependencies:

```toml
[workspace]
members = [
    "domain",
    "application", 
    "infrastructure",
    "presentation-frontend",
    "presentation-backend"
]
```

### 2. Update Development Commands

**Old:**
```bash
cargo tauri dev
```

**New:**
```bash
cd presentation-backend
cargo tauri dev
```

### 3. Update Build Commands

**Old:**
```bash
cargo tauri build
```

**New:**
```bash
cd presentation-backend
cargo tauri build
```

### 4. Import Changes

**Old imports:**
```rust
use crate::domain::entities::*;
use crate::services::ContactApi;
```

**New imports:**
```rust
use domain::{Contact, CreateContactRequest, UpdateContactRequest};
use presentation_frontend::ContactApi;
```

## Benefits of the New Structure

1. **Enforced Dependencies**: Cargo prevents violations of Clean Architecture rules
2. **Better Testing**: Each layer can be tested independently
3. **Modularity**: Easier to replace implementations (e.g., switch databases)
4. **Clarity**: Clear separation of concerns
5. **Reusability**: Domain and application layers can be reused in different contexts

## Troubleshooting

### Build Errors
- Make sure you're running commands from the correct directory
- Check that all workspace members are properly defined in the root `Cargo.toml`

### Import Errors
- Use the crate name (e.g., `domain::`) instead of relative paths (e.g., `crate::domain::`)
- Check that dependencies are properly declared in each crate's `Cargo.toml`

### Tauri Configuration
- The `tauri.conf.json` is now in `presentation-backend/`
- Frontend build commands reference the new frontend location