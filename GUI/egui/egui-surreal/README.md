# SurrealDB GUI Application - Clean Architecture

This project demonstrates a SurrealDB GUI application built with egui and structured using Clean Architecture principles.

## Architecture Overview

The application is organized into four main layers:

### 1. Domain Layer (`src/domain/`)

- **Entities** (`entities.rs`): Core business objects like `Person`, `PersonData`, `AuthParams`, `MessageType`, and `AppMessage`
- **Repository Traits** (`repositories.rs`): Abstract interfaces for data access (`PersonRepository`, `AuthRepository`, `QueryRepository`)

### 2. Application Layer (`src/application/`)

- **Use Cases** (`use_cases/`): Business logic implementations
  - `PersonUseCases`: Person management operations (create, delete, list)
  - `AuthUseCases`: Authentication operations (sign up, sign in, session management)
  - `QueryUseCases`: Raw SurrealQL query execution
- **Services** (`services/`): Application services that coordinate use cases
  - `CommandService`: Handles all application commands through a unified interface

### 3. Infrastructure Layer (`src/infrastructure/`)

- **Database** (`database/`): Concrete implementations of repository traits
  - `SurrealRepository`: SurrealDB implementation of all repository interfaces

### 4. Presentation Layer (`src/presentation/`)

- **UI Components** (`ui/components/`): Individual tab components
  - `people_tab.rs`: People management UI
  - `auth_tab.rs`: Authentication UI
  - `query_tab.rs`: Raw query interface
  - `session_tab.rs`: Session information display
- **Controllers** (`controllers/`): UI logic and state management
  - `AppController`: Manages application state and coordinates with use cases
- **State** (`state/`): UI state management
  - `AppState`: Application state structure
- **App** (`app.rs`): Main application UI coordination and eframe integration

## Key Benefits of This Architecture

1. **Separation of Concerns**: Each layer has a clear responsibility
2. **Testability**: Business logic is isolated and easily testable
3. **Maintainability**: Changes in one layer don't affect others
4. **Flexibility**: Easy to swap implementations (e.g., different databases)
5. **Dependency Inversion**: High-level modules don't depend on low-level modules

## Dependencies Flow

```
Presentation → Application → Domain ← Infrastructure
```

- **Presentation** depends on Application and Domain
- **Application** depends only on Domain
- **Infrastructure** depends only on Domain
- **Domain** has no dependencies (pure business logic)

## Technical Implementation

### Threading Architecture

The application uses a multi-threaded architecture:
- **Main Thread**: Runs the egui UI using eframe
- **Database Thread**: Handles all SurrealDB operations asynchronously using tokio runtime
- **Communication**: Uses `std::sync::mpsc` channels for command/response communication between threads

### Command Pattern

All user interactions are converted into `Command` enum variants:
- `CreatePerson(String)`, `DeletePerson(String)`, `ListPeople`
- `SignUp`, `SignIn(String, String)`, `SignInRoot`, `Session`
- `RawQuery(String)`

### Message System

The UI displays operation results through a message system with:
- `MessageType::Success` (green) and `MessageType::Error` (red) 
- Timestamps showing elapsed time
- Message history (last 10 messages)

## Running the Application

1. Start SurrealDB:
   ```bash
   surreal start --log trace --user root --pass root memory
   ```

2. Run the application:
   ```bash
   cargo run
   ```

3. Test connection (optional):
   ```bash
   cargo run --bin test_connection
   ```

## Features

- **People Management**: Create, delete, and list people with real-time feedback
- **Authentication**: Sign up, sign in (user/root), and session management
- **Raw Queries**: Execute custom SurrealQL queries with result display
- **Session Info**: View current session details and authentication status
- **Real-time Messaging**: Success/error feedback with timestamps
- **Responsive UI**: Non-blocking operations with loading states

## Dependencies

Key dependencies from `Cargo.toml`:
- **egui/eframe**: Modern immediate mode GUI framework
- **surrealdb**: Multi-model database client
- **tokio**: Async runtime for database operations
- **serde**: Serialization/deserialization
- **anyhow**: Error handling

## Project Structure Benefits

The application demonstrates how Clean Architecture principles can be applied to a GUI application:
- **Maintainability**: Clear separation of concerns across layers
- **Testability**: Business logic isolated from UI and database concerns  
- **Flexibility**: Easy to swap implementations (different databases, UI frameworks)
- **Scalability**: Well-organized structure supports feature additions
- **Responsiveness**: Multi-threaded design keeps UI responsive during database operations