// Page modules
pub mod docs;
pub mod home;
pub mod navbar;
pub mod posts;
pub mod todos;
pub mod users;

// Re-export components for easier access
pub use docs::Documents;
pub use home::Home;
pub use navbar::Navbar;
pub use posts::Posts;
pub use todos::Todos;
pub use users::Users;
