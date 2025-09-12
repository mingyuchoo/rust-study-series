pub mod chunker;
pub mod document;
pub mod embedding;
pub mod parser;
pub mod rag;
pub mod resilience;
pub mod vector_search;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod error_integration_tests;

pub use chunker::{ChunkingConfig, DocumentChunker};
pub use document::DocumentService;
pub use embedding::EmbeddingService;
pub use parser::{DocumentParser, ParsedElement};
pub use rag::RAGService;
pub use resilience::{CircuitBreaker, ResilienceConfig, ResilienceService};
pub use vector_search::VectorSearchService;
