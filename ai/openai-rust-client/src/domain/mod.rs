//! Domain Module (Core Layer)
//!
//! This is the innermost layer of the Onion Architecture.
//! It contains the core business entities and repository interfaces.
//!
//! The domain layer has no dependencies on other layers and defines
//! the core business rules and interfaces that other layers implement.

// Business entities
pub mod entities;

// Repository interfaces
pub mod repositories;
