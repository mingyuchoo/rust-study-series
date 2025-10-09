# Clean Architecture Implementation

This document explains the Clean Architecture implementation in this Tauri + Leptos project.

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    Presentation Layer                        │
│  ┌─────────────────────┐    ┌─────────────────────────────┐ │
│  │  Frontend (Leptos)  │    │   Backend (Tauri Handlers) │ │
│  │                     │    │                             │ │
│  │ - UI Components     │    │ - Command Handlers          │ │
│  │ - API Clients       │    │ - Request/Response DTOs     │ │
│  └─────────────────────┘    └─────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────┐
│                   Infrastructure Layer                      │
│                                                             │
│ - Database Implementations (SQLite)                         │
│ - External API Clients                                      │
│ - File System Access                                        │
│ - Configuration                                             │
└─────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────┐
│                   Application Layer                         │
│                                                             │
│ - Use Cases (Business Logic)                                │
│ - Application Services                                      │
│ - Orchestration Logic                                       │
└─────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────┐
│                     Domain Layer                            │
│                                                             │
│ - Entities (Core Business Objects)                          │
│ - Repository Interfaces                                     │
│ - Domain Errors                                             │
│ - Business Rules                                            │
└─────────────────────────────────────────────────────────────┘
```

## Layer Responsibilities

### 1. Domain Layer (`domain/`)
**Purpose**: Contains the core business logic and rules.

**Contents**:
- `entities.rs`: Core business objects (Contact, CreateContactRequest, etc.)
- `errors.rs`: Domain-specific error types
- `repositories.rs`: Repository trait definitions

**Dependencies**: None (pure Rust + basic serialization)

**Rules**:
- No external dependencies except for basic utilities (serde, uuid, etc.)
- Contains only business logic, no infrastructure concerns
- Defines interfaces that outer layers must implement

### 2. Application Layer (`application/`)
**Purpose**: Orchestrates business logic and defines use cases.

**Contents**:
- `usecases/contact_service.rs`: Business logic implementation
- Validation logic
- Transaction coordination

**Dependencies**: Only `domain`

**Rules**:
- Implements business workflows
- Uses repository interfaces from domain
- No knowledge of databases, UI, or external services

### 3. Infrastructure Layer (`infrastructure/`)
**Purpose**: Implements external concerns and technical details.

**Contents**:
- `database/sqlite_contact_repository.rs`: Database implementation
- External API clients
- File system operations
- Configuration management

**Dependencies**: `domain`, `application`, and external libraries (sqlx, etc.)

**Rules**:
- Implements repository interfaces from domain
- Handles all external system interactions
- Contains framework-specific code

### 4. Presentation Layer
**Purpose**: Handles user interface and external communication.

#### Frontend (`presentation-frontend/`)
- Leptos UI components
- API client implementations
- User interaction handling

**Dependencies**: `domain` (for shared types)

#### Backend (`presentation-backend/`)
- Tauri command handlers
- Request/response mapping
- Application orchestration

**Dependencies**: All layers (as the composition root)

## Dependency Rules

The key principle is that **dependencies point inward**:

```
Presentation → Infrastructure → Application → Domain
```

### Allowed Dependencies:
- ✅ Application → Domain
- ✅ Infrastructure → Domain, Application
- ✅ Presentation → Domain, Application, Infrastructure

### Forbidden Dependencies:
- ❌ Domain → Application, Infrastructure, Presentation
- ❌ Application → Infrastructure, Presentation
- ❌ Infrastructure → Presentation

## Benefits

### 1. **Testability**
Each layer can be tested independently:
```rust
// Test domain logic without database
let service = ContactService::new(Arc::new(MockRepository::new()));

// Test infrastructure without business logic
let repo = SqliteContactRepository::new("sqlite::memory:").await?;
```

### 2. **Flexibility**
Easy to swap implementations:
```rust
// Switch from SQLite to PostgreSQL
let repo = Arc::new(PostgresContactRepository::new(url).await?);
let service = ContactService::new(repo);
```

### 3. **Maintainability**
Clear separation of concerns makes code easier to understand and modify.

### 4. **Reusability**
Domain and application layers can be reused in different contexts (web, CLI, mobile).

## Development Workflow

### Adding a New Feature

1. **Start with Domain**: Define entities and repository interfaces
2. **Add Application Logic**: Implement use cases
3. **Implement Infrastructure**: Add database/external service implementations
4. **Update Presentation**: Add UI and API handlers

### Testing Strategy

1. **Unit Tests**: Test domain entities and application services with mocks
2. **Integration Tests**: Test infrastructure implementations
3. **End-to-End Tests**: Test complete workflows through presentation layer

## Common Patterns

### Repository Pattern
```rust
// Domain defines the interface
#[async_trait]
pub trait ContactRepository: Send + Sync {
    async fn create(&self, contact: Contact) -> ContactResult<Contact>;
}

// Infrastructure implements it
impl ContactRepository for SqliteContactRepository {
    async fn create(&self, contact: Contact) -> ContactResult<Contact> {
        // Database-specific implementation
    }
}
```

### Dependency Injection
```rust
// Application layer receives repository through constructor
pub struct ContactService {
    repository: Arc<dyn ContactRepository>,
}

// Presentation layer wires everything together
let repository = Arc::new(SqliteContactRepository::new(url).await?);
let service = ContactService::new(repository);
```

This architecture ensures that your business logic remains independent of external concerns while maintaining clear boundaries and testability.