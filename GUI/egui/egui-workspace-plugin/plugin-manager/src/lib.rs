//! Core system for file conversion
//! 
//! This crate provides the core functionality for the file converter application,
//! including plugin registry management and conversion engine.

pub mod error;
pub mod registry;
pub mod engine;
pub mod loader;

// Re-export commonly used types
pub use error::{ConversionError, ConversionResult};
pub use registry::PluginRegistry;
pub use engine::ConversionEngine;
pub use loader::PluginLoader;
