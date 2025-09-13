use backend::app::AppContainer;
use backend::config::AppConfig;
use backend::monitoring::{PerformanceMonitor, PerformanceTimer};
use backend::services::cache::{CacheManager, EmbeddingCacheKey};
use std::time::Duration;
use tokio::time::Instant;
use tracing::{info, warn};

/// Performance test configuration
struct PerformanceTestConfig {
    pub embedding_iterations: usize,
    pub search_iterations: usize,
    pub rag_iterations: usize,
    pub concurrent_requests: usize,
    pub large_document_size: usize,
}

impl Default for PerformanceTestConfig {
    fn default() -> Self {
        Self {
            embedding_iterations: 50,
            search_iterations: 100,
            rag_iterations: 20,
            concurrent_requests: 10,
            large_document_size: 100_000, // 100KB
        }
    }
}

/// Performance test results
#[derive(Debug)]
struct PerformanceResults {
    pub operation: String,
    pub iterations: usize,
    pub total_duration: Duration,
    pub avg_duration: Duration,
    pub min_duration: Duration,
    pub max_duration: Duration,
    pub throughput_ops_per_sec: f64,
    pub success_rate: f64,
}

impl PerformanceResults {
    fn new(operation: String, durations: Vec<Duration>, failures: usize) -> Self {
        let iterations = durations.len() + failures;
        let total_duration = durations.iter().copied().sum::<Duration>();
        let avg_duration = if durations.is_empty() {
            Duration::ZERO
        } else {
            total_duration / durations.len() as u32
        };
        let min_duration = durations.iter().min().copied().unwrap_or(Duration::ZERO);
        let max_duration = durations.iter().max().copied().unwrap_or(Duration::ZERO);
        let throughput_ops_per_sec = if avg_duration.as_secs_f64() > 0.0 {
            1.0 / avg_duration.as_secs_f64()
        } else {
            0.0
        };
        let success_rate = durations.len() as f64 / iterations as f64;

        Self {
            operation,
            iterations,
            total_duration,
            avg_duration,
            min_duration,
            max_duration,
            throughput_ops_per_sec,
            success_rate,
        }
    }

    fn print_summary(&self) {
        println!("\n=== Performance Test Results: {} ===", self.operation);
        println!("Iterations: {}", self.iterations);
        println!("Success Rate: {:.2}%", self.success_rate * 100.0);
        println!("Total Duration: {:?}", self.total_duration);
        println!("Average Duration: {:?}", self.avg_duration);
        println!("Min Duration: {:?}", self.min_duration);
        println!("Max Duration: {:?}", self.max_duration);
        println!("Throughput: {:.2} ops/sec", self.throughput_ops_per_sec);
        println!("=======================================\n");
    }
}

#[tokio::test]
#[ignore] // Run with --ignored flag for performance testing
async fn test_embedding_generation_performance() {
    let config = PerformanceTestConfig::default();
    let container = setup_test_container().await;

    info!("Starting embedding generation performance test");

    let test_texts: Vec<String> = vec![
        "Short text for testing".to_string(),
        "This is a medium length text that contains multiple sentences and should provide a good test case for embedding generation performance.".to_string(),
        "Long text ".repeat(100), // Very long text
    ];

    for (i, text) in test_texts.iter().enumerate() {
        let mut durations = Vec::new();
        let mut failures = 0;

        for _ in 0 .. config.embedding_iterations {
            let timer = PerformanceTimer::start("embedding_generation");

            match container.embedding_service.generate_embedding(text.as_str()).await {
                | Ok(_) => {
                    durations.push(timer.finish());
                },
                | Err(e) => {
                    timer.finish();
                    failures += 1;
                    warn!("Embedding generation failed: {}", e);
                },
            }
        }

        let results = PerformanceResults::new(format!("Embedding Generation (Text {})", i + 1), durations, failures);
        results.print_summary();

        // Performance assertions
        assert!(results.success_rate > 0.95, "Success rate should be > 95%");
        assert!(results.avg_duration < Duration::from_secs(5), "Average duration should be < 5s");
    }
}

#[tokio::test]
#[ignore] // Run with --ignored flag for performance testing
async fn test_vector_search_performance() {
    let config = PerformanceTestConfig::default();
    let container = setup_test_container().await;

    info!("Starting vector search performance test");

    // First, ensure we have some data to search
    let test_embedding = container
        .embedding_service
        .generate_embedding("test query for search performance")
        .await
        .expect("Failed to generate test embedding");

    let mut durations = Vec::new();
    let mut failures = 0;

    for _ in 0 .. config.search_iterations {
        let timer = PerformanceTimer::start("vector_search");

        match container
            .vector_search_service
            .search_similar(test_embedding.clone(), 10)
            .await
        {
            | Ok(_) => {
                durations.push(timer.finish());
            },
            | Err(e) => {
                timer.finish();
                failures += 1;
                warn!("Vector search failed: {}", e);
            },
        }
    }

    let results = PerformanceResults::new("Vector Search".to_string(), durations, failures);
    results.print_summary();

    // Performance assertions
    assert!(results.success_rate > 0.90, "Success rate should be > 90%");
    assert!(results.avg_duration < Duration::from_millis(500), "Average duration should be < 500ms");
}

#[tokio::test]
#[ignore] // Run with --ignored flag for performance testing
async fn test_rag_pipeline_performance() {
    let config = PerformanceTestConfig::default();
    let container = setup_test_container().await;

    info!("Starting RAG pipeline performance test");

    let test_questions = vec![
        "What is the main topic?",
        "Can you explain the key concepts?",
        "What are the important details mentioned in the documents?",
    ];

    for (i, question) in test_questions.iter().enumerate() {
        let mut durations = Vec::new();
        let mut failures = 0;

        for _ in 0 .. config.rag_iterations {
            let timer = PerformanceTimer::start("rag_pipeline");

            match container.rag_service.answer_question(question.to_string()).await {
                | Ok(_) => {
                    durations.push(timer.finish());
                },
                | Err(e) => {
                    timer.finish();
                    failures += 1;
                    warn!("RAG pipeline failed: {}", e);
                },
            }
        }

        let results = PerformanceResults::new(format!("RAG Pipeline (Question {})", i + 1), durations, failures);
        results.print_summary();

        // Performance assertions
        assert!(results.success_rate > 0.80, "Success rate should be > 80%");
        assert!(results.avg_duration < Duration::from_secs(10), "Average duration should be < 10s");
    }
}

#[tokio::test]
#[ignore] // Run with --ignored flag for performance testing
async fn test_concurrent_request_performance() {
    let config = PerformanceTestConfig::default();
    let container = setup_test_container().await;

    info!("Starting concurrent request performance test");

    let start_time = Instant::now();
    let mut handles = Vec::new();

    // Spawn concurrent embedding requests
    for i in 0 .. config.concurrent_requests {
        let container = container.clone();
        let handle = tokio::spawn(async move {
            let text = format!("Concurrent test text number {}", i);
            let timer = PerformanceTimer::start("concurrent_embedding");

            match container.embedding_service.generate_embedding(&text).await {
                | Ok(_) => {
                    let duration = timer.finish();
                    Ok(duration)
                },
                | Err(e) => {
                    timer.finish();
                    Err(e)
                },
            }
        });
        handles.push(handle);
    }

    // Wait for all requests to complete
    let mut durations = Vec::new();
    let mut failures = 0;

    for handle in handles {
        match handle.await {
            | Ok(Ok(duration)) => durations.push(duration),
            | Ok(Err(e)) => {
                failures += 1;
                warn!("Concurrent request failed: {}", e);
            },
            | Err(e) => {
                failures += 1;
                warn!("Task join failed: {}", e);
            },
        }
    }

    let total_time = start_time.elapsed();

    let results = PerformanceResults::new("Concurrent Requests".to_string(), durations, failures);
    results.print_summary();

    println!("Total wall-clock time for {} concurrent requests: {:?}", config.concurrent_requests, total_time);
    println!(
        "Concurrency benefit: {:.2}x faster than sequential",
        (results.avg_duration * config.concurrent_requests as u32).as_secs_f64() / total_time.as_secs_f64()
    );

    // Performance assertions
    assert!(results.success_rate > 0.90, "Success rate should be > 90%");
    assert!(
        total_time < results.avg_duration * config.concurrent_requests as u32,
        "Concurrent execution should be faster than sequential"
    );
}

#[tokio::test]
#[ignore] // Run with --ignored flag for performance testing
async fn test_large_document_processing_performance() {
    let config = PerformanceTestConfig::default();
    let container = setup_test_container().await;

    info!("Starting large document processing performance test");

    // Generate a large document
    let large_content = "This is a test sentence that will be repeated many times to create a large document. ".repeat(config.large_document_size / 100);

    let timer = PerformanceTimer::start("large_document_processing");

    let result = container
        .document_service
        .process_document(large_content.clone(), "large_test_document.md".to_string())
        .await;

    let duration = timer.finish();

    match result {
        | Ok(_) => {
            println!("\n=== Large Document Processing Results ===");
            println!("Document size: {} characters", large_content.len());
            println!("Processing time: {:?}", duration);
            println!("Processing rate: {:.2} chars/sec", large_content.len() as f64 / duration.as_secs_f64());
            println!("=========================================\n");

            // Performance assertions
            assert!(
                duration < Duration::from_secs(60),
                "Large document processing should complete within 60 seconds"
            );
        },
        | Err(e) => {
            panic!("Large document processing failed: {}", e);
        },
    }
}

#[tokio::test]
#[ignore] // Run with --ignored flag for performance testing
async fn test_cache_performance() {
    info!("Starting cache performance test");

    let cache_manager = CacheManager::new();
    let iterations = 1000;

    // Test embedding cache performance
    let mut cache_durations = Vec::new();
    let mut direct_durations = Vec::new();

    let test_text = "This is a test text for cache performance testing";
    let cache_key = EmbeddingCacheKey::new(test_text, "test-model");
    let test_embedding = vec![0.1; 1536]; // Simulate embedding

    // Warm up the cache
    cache_manager.embedding_cache.put(cache_key.clone(), test_embedding.clone()).await;

    // Test cache hits
    for _ in 0 .. iterations {
        let start = Instant::now();
        let _result = cache_manager.embedding_cache.get(&cache_key).await;
        cache_durations.push(start.elapsed());
    }

    // Test direct operations (simulate expensive computation)
    for _ in 0 .. iterations {
        let start = Instant::now();
        tokio::time::sleep(Duration::from_micros(100)).await; // Simulate computation
        let _result = test_embedding.clone();
        direct_durations.push(start.elapsed());
    }

    let cache_results = PerformanceResults::new("Cache Hits".to_string(), cache_durations, 0);

    let direct_results = PerformanceResults::new("Direct Operations".to_string(), direct_durations, 0);

    cache_results.print_summary();
    direct_results.print_summary();

    let speedup = direct_results.avg_duration.as_nanos() as f64 / cache_results.avg_duration.as_nanos() as f64;
    println!("Cache speedup: {:.2}x faster than direct operations", speedup);

    // Performance assertions
    assert!(speedup > 10.0, "Cache should be at least 10x faster than direct operations");

    // Test cache statistics
    let stats = cache_manager.get_stats().await;
    println!("Cache hit rate: {:.2}%", stats.overall_hit_rate() * 100.0);
    assert!(stats.overall_hit_rate() > 0.5, "Cache hit rate should be > 50%");
}

#[tokio::test]
#[ignore] // Run with --ignored flag for performance testing
async fn test_memory_usage_monitoring() {
    info!("Starting memory usage monitoring test");

    let performance_monitor = PerformanceMonitor::new();
    performance_monitor.start().await.expect("Failed to start performance monitor");

    // Perform some memory-intensive operations
    let mut large_vectors = Vec::new();
    for i in 0 .. 100 {
        let large_vector = vec![i as f32; 10000]; // 40KB per vector
        large_vectors.push(large_vector);

        if i % 10 == 0 {
            let snapshot = performance_monitor
                .get_performance_snapshot()
                .await
                .expect("Failed to get performance snapshot");

            println!("Memory usage at iteration {}: {} MB", i, snapshot.process_memory_mb().unwrap_or(0));

            // Check for memory pressure
            if snapshot.is_memory_pressure() {
                warn!("Memory pressure detected at iteration {}", i);
            }
        }

        tokio::time::sleep(Duration::from_millis(10)).await;
    }

    // Clean up
    drop(large_vectors);

    // Force garbage collection (in a real scenario, this would be automatic)
    tokio::time::sleep(Duration::from_millis(100)).await;

    let final_snapshot = performance_monitor
        .get_performance_snapshot()
        .await
        .expect("Failed to get final performance snapshot");

    println!("Final memory usage: {} MB", final_snapshot.process_memory_mb().unwrap_or(0));

    performance_monitor.stop().await;
}

/// Setup test container for performance tests
async fn setup_test_container() -> AppContainer {
    // Load test configuration
    let config = AppConfig::from_env().expect("Failed to load test configuration");

    // Initialize container
    AppContainer::new(config).await.expect("Failed to initialize test container")
}

/// Benchmark runner for CI/CD integration
#[tokio::test]
#[ignore] // Run with --ignored flag for benchmarking
async fn run_performance_benchmark_suite() {
    println!("=== Running Performance Benchmark Suite ===\n");
    // 벤치마크 모음은 개별 테스트(`#[tokio::test]`)를 직접 호출하지 않습니다.
    // 필요 시, 각 테스트 로직을 별도의 helper 함수로 분리한 뒤 여기서 호출하세요.
    println!("=== Performance Benchmark Suite Complete ===");
}

/// Load test for stress testing
#[tokio::test]
#[ignore] // Run with --ignored flag for load testing
async fn load_test_sustained_throughput() {
    let container = setup_test_container().await;
    let duration = Duration::from_secs(60); // 1 minute load test
    let concurrent_users = 5;

    info!("Starting {} second load test with {} concurrent users", duration.as_secs(), concurrent_users);

    let start_time = Instant::now();
    let mut handles = Vec::new();

    for user_id in 0 .. concurrent_users {
        let container = container.clone();
        let test_duration = duration;

        let handle = tokio::spawn(async move {
            let mut request_count = 0;
            let mut error_count = 0;

            while start_time.elapsed() < test_duration {
                let question = format!("Load test question {} from user {}", request_count, user_id);

                match container.rag_service.answer_question(question).await {
                    | Ok(_) => request_count += 1,
                    | Err(_) => error_count += 1,
                }

                tokio::time::sleep(Duration::from_millis(100)).await; // 10 RPS per user
            }

            (request_count, error_count)
        });

        handles.push(handle);
    }

    // Wait for all users to complete
    let mut total_requests = 0;
    let mut total_errors = 0;

    for handle in handles {
        let (requests, errors) = handle.await.expect("Load test task failed");
        total_requests += requests;
        total_errors += errors;
    }

    let actual_duration = start_time.elapsed();
    let throughput = total_requests as f64 / actual_duration.as_secs_f64();
    let error_rate = total_errors as f64 / (total_requests + total_errors) as f64;

    println!("\n=== Load Test Results ===");
    println!("Duration: {:?}", actual_duration);
    println!("Total Requests: {}", total_requests);
    println!("Total Errors: {}", total_errors);
    println!("Throughput: {:.2} RPS", throughput);
    println!("Error Rate: {:.2}%", error_rate * 100.0);
    println!("========================\n");

    // Performance assertions
    assert!(error_rate < 0.05, "Error rate should be < 5%");
    assert!(throughput > 1.0, "Throughput should be > 1 RPS");
}
