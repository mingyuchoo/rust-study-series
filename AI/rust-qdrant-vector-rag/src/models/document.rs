use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a chunk of a document with its content and metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DocumentChunk {
    pub id: String,
    pub document_id: String,
    pub content: String,
    pub metadata: ChunkMetadata,
    pub embedding: Option<Vec<f32>>,
    pub created_at: DateTime<Utc>,
}

impl DocumentChunk {
    /// Creates a new DocumentChunk with a generated UUID
    pub fn new(document_id: String, content: String, metadata: ChunkMetadata) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            document_id,
            content,
            metadata,
            embedding: None,
            created_at: Utc::now(),
        }
    }

    /// Sets the embedding for this chunk
    pub fn with_embedding(mut self, embedding: Vec<f32>) -> Self {
        self.embedding = Some(embedding);
        self
    }

    /// Returns the token count estimate for this chunk
    #[allow(dead_code)]
    pub fn estimated_token_count(&self) -> usize {
        // Rough estimation: 1 token ≈ 4 characters
        self.content.len() / 4
    }
}

/// Metadata associated with a document chunk
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChunkMetadata {
    pub source_file: String,
    pub chunk_index: usize,
    pub headers: Vec<String>,
    pub chunk_type: ChunkType,
    pub start_position: Option<usize>,
    pub end_position: Option<usize>,
    pub parent_section: Option<String>,
}

impl ChunkMetadata {
    /// Creates new chunk metadata
    pub fn new(source_file: String, chunk_index: usize, chunk_type: ChunkType) -> Self {
        Self {
            source_file,
            chunk_index,
            headers: Vec::new(),
            chunk_type,
            start_position: None,
            end_position: None,
            parent_section: None,
        }
    }

    /// Adds headers to the metadata
    pub fn with_headers(mut self, headers: Vec<String>) -> Self {
        self.headers = headers;
        self
    }

    /// Sets the position range in the original document
    pub fn with_position(mut self, start: usize, end: usize) -> Self {
        self.start_position = Some(start);
        self.end_position = Some(end);
        self
    }

    /// Sets the parent section for this chunk
    #[allow(dead_code)]
    pub fn with_parent_section(mut self, parent: String) -> Self {
        self.parent_section = Some(parent);
        self
    }
}

/// Types of content chunks that can be extracted from markdown
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ChunkType {
    Text,
    CodeBlock,
    List,
    Table,
    Header,
    Quote,
}

impl Default for ChunkType {
    fn default() -> Self {
        ChunkType::Text
    }
}

/// Result from a vector similarity search
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResult {
    pub chunk: DocumentChunk,
    pub relevance_score: f32,
}

impl SearchResult {
    /// Creates a new search result
    pub fn new(chunk: DocumentChunk, relevance_score: f32) -> Self {
        Self { chunk, relevance_score }
    }
}

/// Represents a document identifier
#[allow(dead_code)]
pub type DocumentId = String;
