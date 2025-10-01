// Clean Architecture 계층 구조
pub mod adapters;
pub mod application;
pub mod domain;
pub mod infra;

// 공개 API 재수출
pub use adapters::http::handlers::AppState;
pub use adapters::http::routes::create_router;
pub use application::usecases::*;
pub use domain::entities::*;
pub use infra::azure_embedding_service::AzureEmbeddingService;
pub use infra::database;
pub use infra::sqlite_repository::SqliteEmbeddingRepository;
