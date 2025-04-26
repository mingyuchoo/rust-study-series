//! Application Module (Use Case Layer)
//!
//! This is the second layer of the Onion Architecture.
//! It contains application-specific business logic and use cases.
//!
//! The application layer depends only on the domain layer.
//! It defines ports (interfaces) for communication with external systems
//! and implements services that orchestrate the core domain entities.

// Input/output ports (interfaces)
pub mod ports;

// Application services (use case implementations)
pub mod services;
