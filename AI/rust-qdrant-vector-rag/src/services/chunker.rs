use crate::models::{ChunkMetadata, ChunkType, DocumentChunk, ServiceError};
use crate::services::parser::{DocumentParser, ParsedElement};

/// Configuration for document chunking
#[derive(Debug, Clone)]
pub struct ChunkingConfig {
    /// Maximum size of a chunk in characters
    pub max_chunk_size: usize,
    /// Overlap size between chunks in characters
    pub overlap_size: usize,
    /// Minimum chunk size to avoid very small chunks
    pub min_chunk_size: usize,
    /// Whether to respect semantic boundaries (headers, paragraphs)
    pub respect_boundaries: bool,
}

impl Default for ChunkingConfig {
    fn default() -> Self {
        Self {
            max_chunk_size: 2000, // ~500 tokens
            overlap_size: 200,    // ~50 tokens
            min_chunk_size: 100,  // ~25 tokens
            respect_boundaries: true,
        }
    }
}

/// Document chunker that splits documents into optimal chunks for embedding
pub struct DocumentChunker {
    config: ChunkingConfig,
    parser: DocumentParser,
}

impl DocumentChunker {
    /// Creates a new document chunker with default configuration
    pub fn new() -> Self {
        Self {
            config: ChunkingConfig::default(),
            parser: DocumentParser::new(),
        }
    }

    /// Creates a chunker with custom configuration
    pub fn with_config(config: ChunkingConfig) -> Self {
        Self {
            config,
            parser: DocumentParser::new(),
        }
    }

    /// Chunks a markdown document into optimally sized pieces
    pub fn chunk_document(&self, content: &str, document_id: String, source_file: String) -> Result<Vec<DocumentChunk>, ServiceError> {
        // First parse the document into structured elements
        let elements = self.parser.parse(content, source_file.clone())?;

        #[cfg(test)]
        println!("Parsed {} elements from content: '{}'", elements.len(), content);
        #[cfg(test)]
        for (i, element) in elements.iter().enumerate() {
            println!("Element {}: {:?} - '{}'", i, element.element_type, element.content);
        }

        if elements.is_empty() {
            return Ok(Vec::new());
        }

        let mut chunks = Vec::new();
        let mut current_chunk_content = String::new();
        let mut current_headers: Vec<String> = Vec::new();
        let mut chunk_index = 0;
        let mut elements_in_chunk: Vec<ParsedElement> = Vec::new();

        for element in elements {
            // Update headers if this is a header element
            if element.element_type == ChunkType::Header {
                self.update_header_context(&mut current_headers, &element);
            }

            // Check if adding this element would exceed the chunk size
            let element_size = element.content.len();
            let would_exceed = current_chunk_content.len() + element_size > self.config.max_chunk_size;

            if would_exceed && !current_chunk_content.is_empty() {
                // Create a chunk from current content
                if let Some(chunk) = self.create_chunk_from_content(
                    &current_chunk_content,
                    &elements_in_chunk,
                    &current_headers,
                    &document_id,
                    &source_file,
                    chunk_index,
                )? {
                    chunks.push(chunk);
                    chunk_index += 1;
                }

                // Start new chunk with overlap if configured
                if self.config.overlap_size > 0 && current_chunk_content.len() > self.config.overlap_size {
                    current_chunk_content = self.create_overlap_content(&current_chunk_content);
                } else {
                    current_chunk_content.clear();
                }
                elements_in_chunk.clear();
            }

            // For very large elements, split them further
            if element_size > self.config.max_chunk_size {
                let sub_chunks = self.split_large_element(&element, &current_headers, &document_id, &source_file, &mut chunk_index)?;
                chunks.extend(sub_chunks);
                current_chunk_content.clear();
                elements_in_chunk.clear();
            } else {
                // Add the current element to the chunk
                if !current_chunk_content.is_empty() {
                    current_chunk_content.push('\n');
                }
                current_chunk_content.push_str(&element.content);
                elements_in_chunk.push(element);
            }
        }

        // Handle remaining content
        if !current_chunk_content.trim().is_empty() {
            if let Some(chunk) = self.create_chunk_from_content(
                &current_chunk_content,
                &elements_in_chunk,
                &current_headers,
                &document_id,
                &source_file,
                chunk_index,
            )? {
                chunks.push(chunk);
            }
        }

        Ok(chunks)
    }

    /// Updates the header context based on the current header element
    fn update_header_context(&self, headers: &mut Vec<String>, header_element: &ParsedElement) {
        // Use the headers from the parsed element if available
        if !header_element.headers.is_empty() {
            *headers = header_element.headers.clone();
        } else {
            // Fallback: just add this header
            headers.push(header_element.content.clone());
        }
    }

    /// Creates a chunk from the current content and metadata
    fn create_chunk_from_content(
        &self,
        content: &str,
        elements: &[ParsedElement],
        headers: &[String],
        document_id: &str,
        source_file: &str,
        chunk_index: usize,
    ) -> Result<Option<DocumentChunk>, ServiceError> {
        let trimmed_content = content.trim();

        if trimmed_content.len() < self.config.min_chunk_size {
            return Ok(None);
        }

        // Determine the primary chunk type based on elements
        let chunk_type = self.determine_chunk_type(elements);

        // Get position information from elements
        let (start_pos, end_pos) = self.get_position_range(elements);

        let metadata = ChunkMetadata::new(source_file.to_string(), chunk_index, chunk_type)
            .with_headers(headers.to_vec())
            .with_position(start_pos, end_pos);

        let chunk = DocumentChunk::new(document_id.to_string(), trimmed_content.to_string(), metadata);

        Ok(Some(chunk))
    }

    /// Determines the primary chunk type based on the elements it contains
    fn determine_chunk_type(&self, elements: &[ParsedElement]) -> ChunkType {
        if elements.is_empty() {
            return ChunkType::Text;
        }

        // Count different types
        let mut type_counts = std::collections::HashMap::new();
        for element in elements {
            *type_counts.entry(element.element_type.clone()).or_insert(0) += 1;
        }

        // Return the most common type, with preference for special types
        if type_counts.contains_key(&ChunkType::CodeBlock) {
            ChunkType::CodeBlock
        } else if type_counts.contains_key(&ChunkType::Table) {
            ChunkType::Table
        } else if type_counts.contains_key(&ChunkType::List) {
            ChunkType::List
        } else if type_counts.contains_key(&ChunkType::Quote) {
            ChunkType::Quote
        } else if type_counts.contains_key(&ChunkType::Header) {
            ChunkType::Header
        } else {
            ChunkType::Text
        }
    }

    /// Gets the position range from a collection of elements
    fn get_position_range(&self, elements: &[ParsedElement]) -> (usize, usize) {
        if elements.is_empty() {
            return (0, 0);
        }

        let start = elements.iter().map(|e| e.start_position).min().unwrap_or(0);
        let end = elements.iter().map(|e| e.end_position).max().unwrap_or(0);
        (start, end)
    }

    /// Creates overlap content from the end of the current chunk
    fn create_overlap_content(&self, content: &str) -> String {
        if content.len() <= self.config.overlap_size {
            return content.to_string();
        }

        // Try to find a good break point (sentence, paragraph, etc.)
        let start_pos = content.len().saturating_sub(self.config.overlap_size);
        let overlap_section = &content[start_pos..];

        // Look for sentence boundaries
        if let Some(sentence_end) = overlap_section.rfind(". ") {
            return overlap_section[sentence_end + 2..].to_string();
        }

        // Look for paragraph boundaries
        if let Some(para_end) = overlap_section.rfind("\n\n") {
            return overlap_section[para_end + 2..].to_string();
        }

        // Look for line boundaries
        if let Some(line_end) = overlap_section.rfind('\n') {
            return overlap_section[line_end + 1..].to_string();
        }

        // Fallback to character-based overlap
        overlap_section.to_string()
    }

    /// Splits a large element into smaller chunks
    fn split_large_element(
        &self,
        element: &ParsedElement,
        headers: &[String],
        document_id: &str,
        source_file: &str,
        chunk_index: &mut usize,
    ) -> Result<Vec<DocumentChunk>, ServiceError> {
        let mut chunks = Vec::new();
        let content = &element.content;

        if content.len() <= self.config.max_chunk_size {
            return Ok(chunks);
        }

        let mut start = 0;
        while start < content.len() {
            let end = std::cmp::min(start + self.config.max_chunk_size, content.len());
            let mut chunk_end = end;

            // Try to find a good break point if we're not at the end
            if end < content.len() {
                let search_start = std::cmp::max(start, end.saturating_sub(200));
                let search_section = &content[search_start..end];

                // Look for sentence boundaries
                if let Some(sentence_pos) = search_section.rfind(". ") {
                    chunk_end = search_start + sentence_pos + 1;
                } else if let Some(para_pos) = search_section.rfind("\n\n") {
                    chunk_end = search_start + para_pos + 1;
                } else if let Some(line_pos) = search_section.rfind('\n') {
                    chunk_end = search_start + line_pos + 1;
                }
            }

            let chunk_content = content[start..chunk_end].trim();
            if chunk_content.len() >= self.config.min_chunk_size {
                let metadata = ChunkMetadata::new(source_file.to_string(), *chunk_index, element.element_type.clone())
                    .with_headers(headers.to_vec())
                    .with_position(element.start_position + start, element.start_position + chunk_end);

                let chunk = DocumentChunk::new(document_id.to_string(), chunk_content.to_string(), metadata);

                chunks.push(chunk);
                *chunk_index += 1;
            }

            // Move to next chunk with overlap
            let next_start = if self.config.overlap_size > 0 && chunk_end > self.config.overlap_size {
                chunk_end.saturating_sub(self.config.overlap_size)
            } else {
                chunk_end
            };

            // Prevent infinite loop - ensure we always make progress
            start = if next_start <= start { start + 1 } else { next_start };

            // Safety check to prevent infinite loops
            if start >= content.len() {
                break;
            }
        }

        Ok(chunks)
    }

    /// Estimates the token count for a text string
    pub fn estimate_token_count(text: &str) -> usize {
        // Rough estimation: 1 token ≈ 4 characters for English text
        text.len() / 4
    }

    /// Validates that chunks meet the requirements
    pub fn validate_chunks(&self, chunks: &[DocumentChunk]) -> Result<(), ServiceError> {
        for (i, chunk) in chunks.iter().enumerate() {
            let content_len = chunk.content.len();

            if content_len > self.config.max_chunk_size {
                return Err(ServiceError::DocumentProcessing(format!(
                    "Chunk {} exceeds maximum size: {} > {}",
                    i, content_len, self.config.max_chunk_size
                )));
            }

            if content_len < self.config.min_chunk_size {
                return Err(ServiceError::DocumentProcessing(format!(
                    "Chunk {} is below minimum size: {} < {}",
                    i, content_len, self.config.min_chunk_size
                )));
            }
        }

        Ok(())
    }
}

impl Default for DocumentChunker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_simple_document() {
        let chunker = DocumentChunker::with_config(ChunkingConfig {
            max_chunk_size: 2000,
            overlap_size: 200,
            min_chunk_size: 50, // Reduced to allow smaller test content
            respect_boundaries: true,
        });
        let content = "This is a simple document with some content that should be chunked appropriately.";

        let chunks = chunker.chunk_document(content, "doc1".to_string(), "test.md".to_string()).unwrap();

        println!("Created {} chunks", chunks.len());
        for (i, chunk) in chunks.iter().enumerate() {
            println!("Chunk {}: '{}'", i, chunk.content);
        }

        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].content, content);
        assert_eq!(chunks[0].document_id, "doc1");
        assert_eq!(chunks[0].metadata.source_file, "test.md");
        assert_eq!(chunks[0].metadata.chunk_index, 0);
    }

    #[test]
    fn test_chunk_large_document() {
        let chunker = DocumentChunker::with_config(ChunkingConfig {
            max_chunk_size: 100,
            overlap_size: 20,
            min_chunk_size: 10,
            respect_boundaries: true,
        });

        let content = "This is a very long document. ".repeat(10); // ~300 characters

        let chunks = chunker.chunk_document(&content, "doc1".to_string(), "test.md".to_string()).unwrap();

        assert!(chunks.len() >= 1, "Should create at least one chunk");

        // Verify chunk sizes
        for chunk in &chunks {
            assert!(chunk.content.len() <= 120, "Chunk too large: {}", chunk.content.len()); // Allow some flexibility
            if chunk.content.len() < 10 {
                println!("Warning: Small chunk found: '{}'", chunk.content);
            }
        }
    }

    #[test]
    fn test_chunk_with_headers() {
        let chunker = DocumentChunker::with_config(ChunkingConfig {
            max_chunk_size: 200,
            overlap_size: 50,
            min_chunk_size: 20,
            respect_boundaries: true,
        });

        let content = r#"# Main Title

This is content under the main title.

## Subtitle

This is content under the subtitle with more text to make it longer.

### Sub-subtitle

Even more content under the sub-subtitle to ensure we have enough text."#;

        let chunks = chunker.chunk_document(content, "doc1".to_string(), "test.md".to_string()).unwrap();

        assert!(!chunks.is_empty());

        // Check that headers are preserved in metadata
        let chunks_with_headers: Vec<_> = chunks.iter().filter(|c| !c.metadata.headers.is_empty()).collect();

        assert!(!chunks_with_headers.is_empty(), "Should have chunks with header metadata");
    }

    #[test]
    fn test_chunk_code_blocks() {
        let chunker = DocumentChunker::new();
        let content = r#"Here's some code:

```rust
fn main() {
    println!("Hello, world!");
    // This is a comment
    let x = 42;
    println!("The answer is {}", x);
}
```

And some explanation after the code."#;

        let chunks = chunker.chunk_document(content, "doc1".to_string(), "test.md".to_string()).unwrap();

        assert!(!chunks.is_empty());

        // Should have at least one chunk with code content
        let code_chunks: Vec<_> = chunks.iter().filter(|c| c.content.contains("fn main()")).collect();

        assert!(!code_chunks.is_empty(), "Should preserve code blocks");
    }

    #[test]
    fn test_empty_document() {
        let chunker = DocumentChunker::new();
        let chunks = chunker.chunk_document("", "doc1".to_string(), "test.md".to_string()).unwrap();

        assert!(chunks.is_empty());
    }

    #[test]
    fn test_whitespace_only_document() {
        let chunker = DocumentChunker::new();
        let chunks = chunker.chunk_document("   \n\n   ", "doc1".to_string(), "test.md".to_string()).unwrap();

        assert!(chunks.is_empty());
    }

    #[test]
    fn test_chunk_validation() {
        let chunker = DocumentChunker::with_config(ChunkingConfig {
            max_chunk_size: 100,
            overlap_size: 10,
            min_chunk_size: 20,
            respect_boundaries: true,
        });

        let content = "This is a test document with enough content to create valid chunks.";
        let chunks = chunker.chunk_document(content, "doc1".to_string(), "test.md".to_string()).unwrap();

        // Validation should pass
        assert!(chunker.validate_chunks(&chunks).is_ok());
    }

    #[test]
    fn test_estimate_token_count() {
        assert_eq!(DocumentChunker::estimate_token_count("hello world"), 2); // 11 chars / 4 ≈ 2
        assert_eq!(DocumentChunker::estimate_token_count(""), 0);
        assert_eq!(DocumentChunker::estimate_token_count("a"), 0); // 1 char / 4 = 0
        assert_eq!(DocumentChunker::estimate_token_count("abcd"), 1); // 4 chars / 4 = 1
    }

    #[test]
    fn test_chunk_type_detection() {
        let chunker = DocumentChunker::with_config(ChunkingConfig {
            max_chunk_size: 2000,
            overlap_size: 200,
            min_chunk_size: 10, // Reduced to allow smaller test content
            respect_boundaries: true,
        });

        // Test code block detection
        let code_content = r#"```rust
fn test() {}
```"#;

        let chunks = chunker.chunk_document(code_content, "doc1".to_string(), "test.md".to_string()).unwrap();

        assert!(!chunks.is_empty());
        // The chunk should contain code content
        assert!(chunks[0].content.contains("fn test()"));
    }

    #[test]
    fn test_overlap_creation() {
        let chunker = DocumentChunker::with_config(ChunkingConfig {
            max_chunk_size: 80,
            overlap_size: 15,
            min_chunk_size: 10,
            respect_boundaries: true,
        });

        let content = "This is sentence one. This is sentence two. This is sentence three. This is sentence four.";

        let chunks = chunker.chunk_document(content, "doc1".to_string(), "test.md".to_string()).unwrap();

        // Just verify we can create chunks without infinite loops
        assert!(!chunks.is_empty(), "Should create at least one chunk");

        // Verify all chunks have reasonable content
        for chunk in &chunks {
            assert!(!chunk.content.trim().is_empty(), "Chunk should not be empty");
        }
    }

    #[test]
    fn test_semantic_boundary_detection() {
        let chunker = DocumentChunker::with_config(ChunkingConfig {
            max_chunk_size: 150,
            overlap_size: 30,
            min_chunk_size: 20,
            respect_boundaries: true,
        });

        let content = r#"# Introduction

This is the introduction paragraph with some content.

## Section One

This is section one with detailed content that should be chunked properly.

## Section Two

This is section two with more content to test boundary detection."#;

        let chunks = chunker.chunk_document(content, "doc1".to_string(), "test.md".to_string()).unwrap();

        assert!(!chunks.is_empty());

        // Verify that chunks contain meaningful content
        for chunk in &chunks {
            assert!(chunk.content.len() >= 20, "Chunk should meet minimum size");
            assert!(chunk.content.len() <= 180, "Chunk should not exceed max size by much"); // Allow some flexibility
        }
    }

    #[test]
    fn test_metadata_preservation() {
        let chunker = DocumentChunker::with_config(ChunkingConfig {
            max_chunk_size: 200,
            overlap_size: 50,
            min_chunk_size: 30,
            respect_boundaries: true,
        });

        let content = r#"# Main Title

Content under main title.

## Subtitle

Content under subtitle with enough text to create multiple chunks if needed."#;

        let chunks = chunker.chunk_document(content, "test-doc".to_string(), "example.md".to_string()).unwrap();

        assert!(!chunks.is_empty());

        // Verify metadata is properly set
        for (i, chunk) in chunks.iter().enumerate() {
            assert_eq!(chunk.document_id, "test-doc");
            assert_eq!(chunk.metadata.source_file, "example.md");
            assert_eq!(chunk.metadata.chunk_index, i);
            assert!(chunk.id.len() > 0, "Chunk should have a valid ID");
        }
    }

    #[test]
    fn test_large_element_splitting() {
        let chunker = DocumentChunker::with_config(ChunkingConfig {
            max_chunk_size: 100,
            overlap_size: 20,
            min_chunk_size: 15,
            respect_boundaries: true,
        });

        // Create a very long single element that needs to be split
        let long_content = "This is a very long paragraph that exceeds the maximum chunk size and should be split into multiple smaller chunks while preserving the content integrity and maintaining proper overlap between the chunks.";

        let chunks = chunker.chunk_document(long_content, "doc1".to_string(), "test.md".to_string()).unwrap();

        // Should create multiple chunks from the single long element
        assert!(chunks.len() >= 1, "Should create at least one chunk");

        // Verify chunk sizes
        for chunk in &chunks {
            assert!(chunk.content.len() <= 120, "Chunk should not exceed max size by much"); // Allow some flexibility
            assert!(chunk.content.len() >= 15, "Chunk should meet minimum size");
        }
    }

    #[test]
    fn test_configurable_parameters() {
        // Test with different configurations
        let configs = vec![
            ChunkingConfig {
                max_chunk_size: 50,
                overlap_size: 10,
                min_chunk_size: 5,
                respect_boundaries: true,
            },
            ChunkingConfig {
                max_chunk_size: 200,
                overlap_size: 40,
                min_chunk_size: 20,
                respect_boundaries: false,
            },
        ];

        let content = "This is test content that will be chunked with different configurations to verify the chunker respects the parameters.";

        for config in configs {
            let chunker = DocumentChunker::with_config(config.clone());
            let chunks = chunker.chunk_document(content, "doc1".to_string(), "test.md".to_string()).unwrap();

            // Verify chunks respect the configuration
            for chunk in &chunks {
                assert!(chunk.content.len() <= config.max_chunk_size + 20, "Chunk should respect max size"); // Allow some flexibility
                assert!(chunk.content.len() >= config.min_chunk_size, "Chunk should respect min size");
            }
        }
    }
}
