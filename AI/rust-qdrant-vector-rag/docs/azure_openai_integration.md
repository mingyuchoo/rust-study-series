# Azure OpenAI Client Integration

This document describes the Azure OpenAI client integration implemented for the Rust Qdrant Vector RAG system.

## Overview

The Azure OpenAI client provides a robust, production-ready integration with Azure OpenAI services, supporting:

- **Embedding Generation**: Single and batch text embedding generation using Azure OpenAI's text-embedding models
- **Chat Completions**: GPT-4 and other chat model integrations for answer generation
- **Rate Limiting**: Exponential backoff retry logic with configurable parameters
- **Error Handling**: Comprehensive error handling with proper HTTP status code mapping
- **Authentication**: Secure API key-based authentication
- **Logging**: Structured logging for monitoring and debugging

## Architecture

### Components

1. **AzureOpenAIClient**: Core client for direct API interactions
2. **EmbeddingService**: High-level service interface for embedding operations
3. **Configuration**: Environment-based configuration management
4. **Error Handling**: Custom error types with proper propagation

### Key Features

#### Retry Logic with Exponential Backoff
- Configurable maximum retry attempts (default: 3)
- Exponential backoff with jitter (1s, 2s, 4s, 8s, ...)
- Maximum delay cap of 30 seconds
- Automatic retry on network errors and rate limits

#### Comprehensive Error Handling
- Network errors (timeouts, connection failures)
- Authentication errors (invalid API keys)
- Rate limiting (429 responses)
- Validation errors (malformed requests)
- API errors (service unavailable, etc.)

#### Structured Logging
- Request/response logging for debugging
- Performance metrics (response times, token usage)
- Error tracking with context
- Configurable log levels

## Configuration

### Environment Variables

```bash
# Required
AZURE_OPENAI_ENDPOINT=https://your-resource.openai.azure.com
AZURE_OPENAI_API_KEY=your-api-key-here
AZURE_OPENAI_CHAT_DEPLOYMENT=gpt-4
AZURE_OPENAI_EMBED_DEPLOYMENT=text-embedding-3-large

# Optional (with defaults)
AZURE_OPENAI_API_VERSION=2024-02-01
AZURE_OPENAI_MAX_RETRIES=3
AZURE_OPENAI_TIMEOUT_SECONDS=60
```

### Configuration Validation

The system validates all configuration at startup:
- Endpoint must be a valid HTTPS URL
- API key must be properly formatted
- Deployment names must be non-empty
- Timeout and retry values must be within reasonable ranges

## Usage Examples

### Basic Embedding Generation

```rust
use rust_qdrant_vector_rag::clients::AzureOpenAIClient;
use rust_qdrant_vector_rag::config::AzureOpenAIConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration from environment
    let config = AzureOpenAIConfig::from_env()?;
    
    // Create client
    let client = AzureOpenAIClient::new(config)?;
    
    // Generate embedding
    let embedding = client.generate_embedding("Hello, world!").await?;
    println!("Generated embedding with {} dimensions", embedding.len());
    
    Ok(())
}
```

### Batch Embedding Generation

```rust
let texts = vec![
    "First document content",
    "Second document content", 
    "Third document content"
];

let embeddings = client.generate_embeddings_batch(texts).await?;
println!("Generated {} embeddings", embeddings.len());
```

### Using the Embedding Service

```rust
use rust_qdrant_vector_rag::services::embedding::{EmbeddingService, EmbeddingServiceImpl};

let embedding_service = EmbeddingServiceImpl::new(azure_client);

// Single embedding with validation
let embedding = embedding_service.generate_embedding("test text").await?;

// Batch embeddings with validation
let embeddings = embedding_service.generate_embeddings_batch(texts).await?;
```

### Chat Completion

```rust
use rust_qdrant_vector_rag::clients::azure_openai::ChatMessage;

let messages = vec![
    ChatMessage {
        role: "user".to_string(),
        content: "Explain vector embeddings briefly.".to_string(),
    }
];

let response = client.generate_chat_completion(
    messages,
    Some(100), // max_tokens
    Some(0.7)  // temperature
).await?;

println!("Response: {}", response);
```

## Error Handling

### Error Types

The client uses a comprehensive error system:

```rust
pub enum ServiceError {
    EmbeddingGeneration(String),  // Embedding API failures
    ExternalAPI(String),          // General API errors
    Authentication(String),       // Auth failures
    RateLimit(String),           // Rate limiting
    Network(String),             // Network issues
    Validation(String),          // Input validation
    Configuration(String),       // Config errors
    Serialization(String),       // JSON parsing errors
}
```

### Error Handling Best Practices

```rust
match client.generate_embedding("text").await {
    Ok(embedding) => {
        // Process successful result
        println!("Success: {} dimensions", embedding.len());
    }
    Err(ServiceError::RateLimit(msg)) => {
        // Handle rate limiting - maybe wait and retry
        eprintln!("Rate limited: {}", msg);
    }
    Err(ServiceError::Authentication(msg)) => {
        // Handle auth errors - check API key
        eprintln!("Auth error: {}", msg);
    }
    Err(ServiceError::Network(msg)) => {
        // Handle network errors - maybe retry
        eprintln!("Network error: {}", msg);
    }
    Err(e) => {
        // Handle other errors
        eprintln!("Error: {}", e);
    }
}
```

## Testing

### Unit Tests

The implementation includes comprehensive unit tests:

```bash
# Run all client tests
cargo test --lib clients

# Run specific test modules
cargo test --lib clients::tests
cargo test --lib services::tests
```

### Integration Tests

Integration tests require real Azure OpenAI credentials:

```bash
# Set up test environment
export TEST_AZURE_OPENAI_ENDPOINT=https://your-test-resource.openai.azure.com
export TEST_AZURE_OPENAI_API_KEY=your-test-api-key
export TEST_AZURE_OPENAI_CHAT_DEPLOYMENT=gpt-4
export TEST_AZURE_OPENAI_EMBED_DEPLOYMENT=text-embedding-3-large

# Run integration tests
cargo test --lib clients::tests::integration_tests -- --ignored
```

### Example Testing

```bash
# Run the example with real credentials
export AZURE_OPENAI_ENDPOINT=https://your-resource.openai.azure.com
export AZURE_OPENAI_API_KEY=your-api-key
export AZURE_OPENAI_CHAT_DEPLOYMENT=gpt-4
export AZURE_OPENAI_EMBED_DEPLOYMENT=text-embedding-3-large

cargo run --example azure_openai_example
```

## Performance Considerations

### Batch Processing
- Use batch embedding generation for multiple texts
- Batch requests are more efficient than individual requests
- Optimal batch size: 10-50 texts depending on text length

### Connection Management
- HTTP client uses connection pooling automatically
- Configurable timeout settings for different environments
- Proper resource cleanup on client drop

### Memory Usage
- Embeddings are returned as `Vec<f32>` for efficiency
- Large batches may require memory management
- Consider streaming for very large datasets

## Security

### API Key Management
- Store API keys in environment variables only
- Never log API keys or include them in error messages
- Use different keys for different environments

### Network Security
- All requests use HTTPS
- Certificate validation is enforced
- Timeout settings prevent hanging connections

### Error Message Sanitization
- Error messages don't expose sensitive information
- API responses are sanitized before logging
- User-facing errors are generic and safe

## Monitoring and Observability

### Logging

The client provides structured logging at multiple levels:

```rust
// Enable debug logging
RUST_LOG=rust_qdrant_vector_rag=debug cargo run

// Enable trace logging for detailed request/response info
RUST_LOG=rust_qdrant_vector_rag=trace cargo run
```

### Metrics

Key metrics to monitor:
- Request latency (embedding generation time)
- Error rates by type
- Retry attempt frequency
- Token usage (from API responses)

### Health Checks

```rust
// Test connectivity
match client.test_connectivity().await {
    Ok(()) => println!("Azure OpenAI is healthy"),
    Err(e) => println!("Azure OpenAI health check failed: {}", e),
}
```

## Troubleshooting

### Common Issues

1. **Authentication Errors**
   - Verify API key is correct and active
   - Check endpoint URL format
   - Ensure deployment names match Azure configuration

2. **Rate Limiting**
   - Implement exponential backoff (built-in)
   - Consider request batching
   - Monitor quota usage in Azure portal

3. **Network Timeouts**
   - Increase timeout settings for slow networks
   - Check firewall/proxy settings
   - Verify DNS resolution

4. **Embedding Dimension Mismatches**
   - Verify deployment model type (text-embedding-3-large = 3072 dimensions)
   - Check model version consistency
   - Validate vector storage configuration

### Debug Mode

Enable detailed logging for troubleshooting:

```bash
RUST_LOG=rust_qdrant_vector_rag=trace cargo run
```

This will show:
- HTTP request/response details
- Retry attempts and delays
- Token usage statistics
- Performance metrics

## Future Enhancements

Potential improvements for future versions:

1. **Streaming Support**: For large text processing
2. **Caching Layer**: Redis integration for embedding caching
3. **Metrics Export**: Prometheus metrics integration
4. **Circuit Breaker**: Automatic failure detection and recovery
5. **Load Balancing**: Multiple endpoint support
6. **Async Batching**: Automatic request batching optimization

## API Reference

### AzureOpenAIClient

#### Methods

- `new(config: AzureOpenAIConfig) -> Result<Self, ServiceError>`
- `generate_embedding(text: &str) -> Result<Vec<f32>, ServiceError>`
- `generate_embeddings_batch(texts: Vec<&str>) -> Result<Vec<Vec<f32>>, ServiceError>`
- `generate_chat_completion(messages: Vec<ChatMessage>, max_tokens: Option<u32>, temperature: Option<f32>) -> Result<String, ServiceError>`
- `test_connectivity() -> Result<(), ServiceError>`

### EmbeddingService

#### Methods

- `generate_embedding(text: &str) -> Result<Vec<f32>, ServiceError>`
- `generate_embeddings_batch(texts: Vec<&str>) -> Result<Vec<Vec<f32>>, ServiceError>`

### Configuration Types

- `AzureOpenAIConfig`: Main configuration structure
- `ChatMessage`: Chat message structure for completions
- `ServiceError`: Comprehensive error enumeration

For detailed API documentation, run:
```bash
cargo doc --open
```