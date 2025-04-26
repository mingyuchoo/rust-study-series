//! Infrastructure Module (Adapter Layer)
//!
//! This is the third layer of the Onion Architecture.
//! It contains concrete implementations of interfaces defined in the domain and
//! application layers.
//!
//! The infrastructure layer depends on the domain and application layers.
//! It provides concrete implementations for repository interfaces and external
//! system adapters. This layer handles all external concerns like databases,
//! APIs, file systems, etc.

// External system adapters (implements application ports)
pub mod adapters;

// Configuration and environment settings
pub mod config;
