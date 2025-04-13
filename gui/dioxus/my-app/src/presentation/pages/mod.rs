// Page modules
pub mod home;
pub mod users;
pub mod todos;
pub mod posts;
pub mod navbar;

// Re-export components for easier access
pub use home::Home;
pub use users::Users;
pub use todos::Todos;
pub use posts::Posts;
pub use navbar::Navbar;
