// lib.rs - Export modules

// Domain layer (innermost layer)
pub mod domain {
    pub mod services;
}

// Application layer
pub mod application {
    pub mod services;
}

// Infrastructure layer (outermost layer)
pub mod infrastructure {
    pub mod repositories;
    pub mod api;
}
