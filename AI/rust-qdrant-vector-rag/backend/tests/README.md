# Integration Tests

This directory contains comprehensive end-to-end integration tests for the Rust Qdrant Vector RAG system.

## Test Structure

### Test Files

- `integration_startup_tests.rs` - Basic startup and configuration tests
- `integration_e2e_tests.rs` - End-to-end service layer tests
- `integration_http_tests.rs` - HTTP API endpoint tests
- `integration_test_runner.rs` - Comprehensive test orchestrator
- `common/mod.rs` - Shared test utilities and helpers

### Test Categories

1. **Environment Setup and Isolation**
   - Test configuration loading
   - Service initialization
   - Collection isolation between tests

2. **Document Processing Pipeline**
   - Markdown parsing and chunking
   - Embedding generation
   - Vector storage

3. **Question Answering Pipeline**
   - Query embedding generation
   - Similarity search
   - Answer generation with sources

4. **Error Handling and Edge Cases**
   - Invalid inputs
   - Service failures
   - Timeout scenarios

5. **Performance and Scalability**
   - Large document processing
   - Query response times
   - Memory usage

6. **Concurrent Operations**
   - Parallel document uploads
   - Concurrent queries
   - Resource contention

7. **Data Cleanup and Isolation**
   - Test data isolation
   - Collection management
   - Resource cleanup

## Prerequisites

### External Services

The integration tests require the following external services to be running:

1. **Qdrant Vector Database**
   ```bash
   docker run -p 6333:6333 qdrant/qdrant
   ```

2. **Azure OpenAI Service**
   - Valid Azure OpenAI endpoint and API key
   - Deployed models: `gpt-4` and `text-embedding-3-large`

### Environment Variables

Set the following environment variables for real integration testing:

```bash
# Server Configuration
export SERVER_HOST=127.0.0.1
export SERVER_PORT=8080
export SERVER_MAX_REQUEST_SIZE=1048576
export SERVER_TIMEOUT_SECONDS=30

# Azure OpenAI Configuration
export AZURE_OPENAI_ENDPOINT=https://your-resource.openai.azure.com
export AZURE_OPENAI_API_KEY=your-api-key-here
export AZURE_OPENAI_API_VERSION=2024-02-01
export AZURE_OPENAI_CHAT_DEPLOYMENT=gpt-4
export AZURE_OPENAI_EMBED_DEPLOYMENT=text-embedding-3-large
export AZURE_OPENAI_MAX_RETRIES=3
export AZURE_OPENAI_TIMEOUT_SECONDS=60

# Qdrant Configuration
export QDRANT_URL=http://localhost:6333
export QDRANT_COLLECTION_NAME=test_documents
export QDRANT_VECTOR_SIZE=3072
export QDRANT_TIMEOUT_SECONDS=30
export QDRANT_MAX_RETRIES=3
```

## Running Tests

### All Tests (Unit + Integration)

```bash
cargo test
```

### Integration Tests Only

```bash
cargo test --test integration_
```

### Specific Test Files

```bash
# Startup tests (no external services required)
cargo test --test integration_startup_tests

# End-to-end tests (requires external services)
cargo test --test integration_e2e_tests --ignored

# HTTP API tests (requires running server)
cargo test --test integration_http_tests --ignored

# Comprehensive test runner (requires external services)
cargo test --test integration_test_runner --ignored
```

### Running with Logs

```bash
RUST_LOG=debug cargo test --test integration_e2e_tests --ignored -- --nocapture
```

## Test Configuration

### Test Collections

Each test suite uses isolated Qdrant collections to prevent interference:

- `test_e2e_documents` - End-to-end tests
- `test_http_documents` - HTTP API tests
- `test_isolation_1`, `test_isolation_2` - Isolation tests
- `test_document_processing` - Document processing tests
- `test_qa_pipeline` - Question answering tests
- `test_error_handling` - Error handling tests
- `test_performance` - Performance tests
- `test_concurrency` - Concurrency tests
- `test_cleanup` - Cleanup tests

### Test Data

The tests use structured test data including:

- Sample markdown documents with various elements
- Predefined test questions
- Large documents for performance testing
- Edge case scenarios

## Test Scenarios

### Document Upload Tests

- ✅ Valid markdown files
- ✅ Invalid file formats
- ✅ Empty content validation
- ✅ Large document handling
- ✅ Concurrent uploads
- ✅ Multipart form uploads
- ✅ JSON payload uploads

### Query Tests

- ✅ Simple questions
- ✅ Complex queries
- ✅ Empty question validation
- ✅ Long question handling
- ✅ Concurrent queries
- ✅ JSON and GET endpoints

### Error Handling Tests

- ✅ Network failures
- ✅ Service unavailability
- ✅ Invalid configurations
- ✅ Rate limiting
- ✅ Timeout scenarios
- ✅ Malformed requests

### Performance Tests

- ✅ Document processing time
- ✅ Query response time
- ✅ Memory usage monitoring
- ✅ Concurrent operation handling
- ✅ Large document processing

## Expected Results

### Performance Benchmarks

- Document processing: < 30 seconds for test documents
- Query response time: < 15 seconds
- Large document processing: < 60 seconds
- Concurrent operations: Support 3+ parallel operations

### Success Criteria

- **Unit Tests**: 100% pass rate
- **Integration Tests**: 70% pass rate (allows for external service issues)
- **Error Handling**: 80% pass rate
- **Performance Tests**: Meet timing requirements

## Troubleshooting

### Common Issues

1. **External Services Not Available**
   ```
   Error: Failed to initialize app container: Failed to create Qdrant client
   ```
   - Ensure Qdrant is running on localhost:6333
   - Check Azure OpenAI credentials and endpoint

2. **Collection Already Exists**
   ```
   Error: Collection already exists
   ```
   - Tests use isolated collections, this should not occur
   - Manually clean up test collections if needed

3. **Timeout Errors**
   ```
   Error: Operation timed out
   ```
   - Increase timeout values in test configuration
   - Check network connectivity to external services

4. **Memory Issues**
   ```
   Error: Out of memory
   ```
   - Reduce test document sizes
   - Run tests sequentially instead of in parallel

### Debug Mode

Enable debug logging for detailed test execution:

```bash
RUST_LOG=backend=debug,integration=debug cargo test --test integration_e2e_tests --ignored -- --nocapture
```

### Test Isolation

If tests interfere with each other:

1. Ensure each test uses a unique collection name
2. Verify cleanup functions are working
3. Run tests sequentially: `cargo test -- --test-threads=1`

## Continuous Integration

### GitHub Actions Example

```yaml
name: Integration Tests

on: [push, pull_request]

jobs:
  integration:
    runs-on: ubuntu-latest
    
    services:
      qdrant:
        image: qdrant/qdrant
        ports:
          - 6333:6333
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Run Integration Tests
        env:
          AZURE_OPENAI_ENDPOINT: ${{ secrets.AZURE_OPENAI_ENDPOINT }}
          AZURE_OPENAI_API_KEY: ${{ secrets.AZURE_OPENAI_API_KEY }}
          QDRANT_URL: http://localhost:6333
        run: cargo test --test integration_e2e_tests --ignored
```

## Contributing

When adding new integration tests:

1. Use the `common` module for shared utilities
2. Follow the naming convention: `test_<feature>_<scenario>`
3. Include proper error handling and logging
4. Use isolated test collections
5. Add cleanup procedures
6. Document expected behavior and performance criteria

## Test Coverage

The integration tests cover:

- ✅ All API endpoints
- ✅ Service layer components
- ✅ External service integrations
- ✅ Error scenarios
- ✅ Performance characteristics
- ✅ Concurrent operations
- ✅ Data isolation
- ✅ Health monitoring

For detailed coverage reports:

```bash
cargo tarpaulin --ignore-tests --out Html
```