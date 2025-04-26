//! OpenAI Rust Client Library
//! Implements a client for the OpenAI API using strict Onion Architecture
//!
//! The architecture follows a strict unidirectional dependency flow:
//! domain → application → infrastructure → presentation

// 1. Domain Layer (Core)
// Contains business entities and interfaces
// Has no dependencies on other layers
pub mod domain;

// 2. Application Layer
// Contains business logic and use cases
// Only depends on the domain layer
pub mod application;

// 3. Infrastructure Layer
// Contains implementations of interfaces defined in the domain and application
// layers Depends on domain and application layers
pub mod infrastructure;

// 4. Presentation Layer
// Contains UI and API endpoints
// Depends on domain, application, and infrastructure layers
pub mod presentation;
