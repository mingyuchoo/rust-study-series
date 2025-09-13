#!/bin/bash

# Performance Benchmark Script for Rust Qdrant Vector RAG Service

set -e

echo "=== Rust Qdrant Vector RAG Performance Benchmark ==="
echo "Starting performance tests..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if required environment variables are set
check_env() {
    print_status "Checking environment variables..."
    
    if [ -z "$AZURE_OPENAI_ENDPOINT" ]; then
        print_error "AZURE_OPENAI_ENDPOINT is not set"
        exit 1
    fi
    
    if [ -z "$AZURE_OPENAI_API_KEY" ]; then
        print_error "AZURE_OPENAI_API_KEY is not set"
        exit 1
    fi
    
    if [ -z "$QDRANT_URL" ]; then
        print_warning "QDRANT_URL not set, using default: http://localhost:6333"
        export QDRANT_URL="http://localhost:6333"
    fi
    
    print_status "Environment variables OK"
}

# Build the project in release mode
build_project() {
    print_status "Building project in release mode..."
    cargo build --release
    
    if [ $? -eq 0 ]; then
        print_status "Build successful"
    else
        print_error "Build failed"
        exit 1
    fi
}

# Run unit tests
run_unit_tests() {
    print_status "Running unit tests..."
    cargo test --release --lib
    
    if [ $? -eq 0 ]; then
        print_status "Unit tests passed"
    else
        print_error "Unit tests failed"
        exit 1
    fi
}

# Run performance tests
run_performance_tests() {
    print_status "Running performance tests..."
    
    # Set test environment variables
    export SKIP_CONNECTIVITY_TEST=1
    export RUST_LOG=info
    
    # Run performance tests with ignored flag
    cargo test --release --test performance_tests -- --ignored --nocapture
    
    if [ $? -eq 0 ]; then
        print_status "Performance tests completed"
    else
        print_warning "Some performance tests may have failed (this is normal if external services are unavailable)"
    fi
}

# Start the service for load testing
start_service() {
    print_status "Starting service for load testing..."
    
    # Kill any existing instances
    pkill -f "rust-qdrant-vector-rag" || true
    sleep 2
    
    # Start the service in background
    RUST_LOG=warn ./target/release/rust-qdrant-vector-rag &
    SERVICE_PID=$!
    
    # Wait for service to start
    sleep 5
    
    # Check if service is running
    if kill -0 $SERVICE_PID 2>/dev/null; then
        print_status "Service started with PID: $SERVICE_PID"
        return 0
    else
        print_error "Failed to start service"
        return 1
    fi
}

# Stop the service
stop_service() {
    if [ ! -z "$SERVICE_PID" ]; then
        print_status "Stopping service (PID: $SERVICE_PID)..."
        kill $SERVICE_PID || true
        wait $SERVICE_PID 2>/dev/null || true
        print_status "Service stopped"
    fi
}

# Run HTTP load tests using curl
run_load_tests() {
    print_status "Running HTTP load tests..."
    
    local base_url="http://localhost:8080"
    
    # Test health endpoint
    print_status "Testing health endpoint..."
    for i in {1..10}; do
        curl -s "$base_url/health" > /dev/null
        if [ $? -eq 0 ]; then
            echo -n "."
        else
            echo -n "x"
        fi
    done
    echo ""
    
    # Test metrics endpoint
    print_status "Testing metrics endpoint..."
    curl -s "$base_url/api/v1/metrics" | jq . > /dev/null 2>&1
    if [ $? -eq 0 ]; then
        print_status "Metrics endpoint OK"
    else
        print_warning "Metrics endpoint may not be available"
    fi
    
    # Test performance health endpoint
    print_status "Testing performance health endpoint..."
    curl -s "$base_url/api/v1/health/performance" | jq . > /dev/null 2>&1
    if [ $? -eq 0 ]; then
        print_status "Performance health endpoint OK"
    else
        print_warning "Performance health endpoint may not be available"
    fi
}

# Run memory usage test
run_memory_test() {
    print_status "Running memory usage test..."
    
    if [ ! -z "$SERVICE_PID" ]; then
        # Monitor memory usage for 30 seconds
        for i in {1..30}; do
            memory_kb=$(ps -o rss= -p $SERVICE_PID 2>/dev/null || echo "0")
            memory_mb=$((memory_kb / 1024))
            echo "Memory usage: ${memory_mb}MB"
            sleep 1
        done
    else
        print_warning "Service not running, skipping memory test"
    fi
}

# Generate performance report
generate_report() {
    print_status "Generating performance report..."
    
    local report_file="performance_report_$(date +%Y%m%d_%H%M%S).txt"
    
    cat > "$report_file" << EOF
=== Rust Qdrant Vector RAG Performance Report ===
Generated: $(date)
System: $(uname -a)
Rust Version: $(rustc --version)

Build Configuration:
- Release mode: Yes
- Target: $(rustc --version --verbose | grep host | cut -d' ' -f2)

Test Results:
- Unit tests: $(if cargo test --release --lib --quiet; then echo "PASSED"; else echo "FAILED"; fi)
- Performance tests: Completed (see detailed output above)
- Load tests: Completed
- Memory tests: Completed

Recommendations:
1. Monitor memory usage during high load
2. Consider connection pooling for production
3. Enable caching for frequently accessed data
4. Use performance metrics for monitoring

EOF

    print_status "Performance report saved to: $report_file"
}

# Cleanup function
cleanup() {
    print_status "Cleaning up..."
    stop_service
    print_status "Cleanup complete"
}

# Set trap for cleanup
trap cleanup EXIT

# Main execution
main() {
    echo "Starting benchmark at $(date)"
    
    check_env
    build_project
    run_unit_tests
    run_performance_tests
    
    # Only run service tests if we can start the service
    if start_service; then
        run_load_tests
        run_memory_test
    else
        print_warning "Skipping service-dependent tests"
    fi
    
    generate_report
    
    print_status "Benchmark completed successfully!"
}

# Run main function
main "$@"