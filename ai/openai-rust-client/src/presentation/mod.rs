//! Presentation Module (Interface Layer)
//!
//! This is the outermost layer of the Onion Architecture.
//! It contains all UI components, API controllers, and endpoints.
//!
//! The presentation layer depends on the domain, application, and
//! infrastructure layers. It handles the interaction with users and external
//! systems, converting requests into application layer calls and formatting
//! responses for clients.

// API controllers and models
pub mod api;

// Web UI components and handlers
pub mod web;
