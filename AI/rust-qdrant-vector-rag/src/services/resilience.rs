use crate::models::ServiceError;
use std::future::Future;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::{sleep, timeout};
use tracing::{debug, error, info, warn};

/// Configuration for resilience patterns
#[derive(Debug, Clone)]
pub struct ResilienceConfig {
    /// Maximum number of retry attempts
    pub max_retries: usize,
    /// Base delay between retries in milliseconds
    pub base_delay_ms: u64,
    /// Maximum delay between retries in milliseconds
    pub max_delay_ms: u64,
    /// Multiplier for exponential backoff
    pub backoff_multiplier: f64,
    /// Timeout for individual operations in seconds
    pub operation_timeout_seconds: u64,
    /// Whether to add jitter to retry delays
    pub use_jitter: bool,
}

impl Default for ResilienceConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay_ms: 1000,
            max_delay_ms: 30000,
            backoff_multiplier: 2.0,
            operation_timeout_seconds: 60,
            use_jitter: true,
        }
    }
}

/// Service for implementing resilience patterns like retry, timeout, and circuit breaker
#[derive(Clone)]
pub struct ResilienceService {
    config: ResilienceConfig,
}

impl ResilienceService {
    pub fn new(config: ResilienceConfig) -> Self {
        Self { config }
    }

    #[allow(dead_code)]
    pub fn with_default_config() -> Self {
        Self::new(ResilienceConfig::default())
    }

    /// Executes an operation with retry logic and exponential backoff
    pub async fn retry_with_backoff<F, Fut, T>(&self, operation: F) -> Result<T, ServiceError>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<T, ServiceError>>,
    {
        let mut attempt = 0;
        let mut last_error = ServiceError::internal("No attempts made");

        while attempt <= self.config.max_retries {
            let start_time = Instant::now();

            debug!(attempt = attempt, max_retries = self.config.max_retries, "Executing operation attempt");

            match operation().await {
                | Ok(result) => {
                    if attempt > 0 {
                        info!(
                            attempt = attempt,
                            duration_ms = start_time.elapsed().as_millis(),
                            "Operation succeeded after retry"
                        );
                    }
                    return Ok(result);
                },
                | Err(error) => {
                    last_error = error.clone();

                    // Check if error is retryable
                    if !error.is_retryable() {
                        warn!(
                            attempt = attempt,
                            error = %error,
                            error_category = error.category(),
                            "Operation failed with non-retryable error"
                        );
                        return Err(error);
                    }

                    if attempt >= self.config.max_retries {
                        error!(
                            attempt = attempt,
                            max_retries = self.config.max_retries,
                            error = %error,
                            error_category = error.category(),
                            "Operation failed after all retry attempts"
                        );
                        break;
                    }

                    // Calculate delay for next attempt
                    let delay = self.calculate_delay(attempt);

                    warn!(
                        attempt = attempt,
                        error = %error,
                        error_category = error.category(),
                        delay_ms = delay.as_millis(),
                        "Operation failed, retrying after delay"
                    );

                    sleep(delay).await;
                    attempt += 1;
                },
            }
        }

        Err(last_error)
    }

    #[allow(dead_code)]
    /// Executes an operation with a timeout
    pub async fn with_timeout<F, Fut, T>(&self, operation: F) -> Result<T, ServiceError>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<T, ServiceError>>,
    {
        let timeout_duration = Duration::from_secs(self.config.operation_timeout_seconds);

        debug!(timeout_seconds = self.config.operation_timeout_seconds, "Executing operation with timeout");

        match timeout(timeout_duration, operation()).await {
            | Ok(result) => result,
            | Err(_) => {
                error!(timeout_seconds = self.config.operation_timeout_seconds, "Operation timed out");
                Err(ServiceError::network(format!(
                    "Operation timed out after {} seconds",
                    self.config.operation_timeout_seconds
                )))
            },
        }
    }

    #[allow(dead_code)]
    /// Executes an operation with both retry and timeout
    pub async fn retry_with_timeout<F, Fut, T>(&self, operation: F) -> Result<T, ServiceError>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<T, ServiceError>>,
    {
        let timeout_duration = Duration::from_secs(self.config.operation_timeout_seconds);

        self.retry_with_backoff(|| {
            let op = &operation;
            async move {
                match tokio::time::timeout(timeout_duration, op()).await {
                    | Ok(result) => result,
                    | Err(_) => Err(ServiceError::network(format!(
                        "Operation timed out after {} seconds",
                        timeout_duration.as_secs()
                    ))),
                }
            }
        })
        .await
    }

    /// Calculates the delay for the next retry attempt using exponential backoff
    fn calculate_delay(&self, attempt: usize) -> Duration {
        let base_delay = self.config.base_delay_ms as f64;
        let multiplier = self.config.backoff_multiplier;

        // Calculate exponential backoff delay
        let delay_ms = base_delay * multiplier.powi(attempt as i32);
        let delay_ms = delay_ms.min(self.config.max_delay_ms as f64);

        // Add jitter if enabled
        let final_delay_ms = if self.config.use_jitter { self.add_jitter(delay_ms) } else { delay_ms };

        Duration::from_millis(final_delay_ms as u64)
    }

    /// Adds random jitter to prevent thundering herd problem
    fn add_jitter(&self, delay_ms: f64) -> f64 {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        // Add ±25% jitter
        let jitter_factor = rng.gen_range(0.75..=1.25);
        delay_ms * jitter_factor
    }

    #[allow(dead_code)]
    /// Executes an operation with graceful degradation
    pub async fn with_fallback<F, Fut, T, FB, FutB>(&self, primary_operation: F, fallback_operation: FB) -> Result<T, ServiceError>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<T, ServiceError>>,
        FB: Fn() -> FutB,
        FutB: Future<Output = Result<T, ServiceError>>,
    {
        debug!("Executing primary operation with fallback");

        match self.retry_with_backoff(&primary_operation).await {
            | Ok(result) => {
                debug!("Primary operation succeeded");
                Ok(result)
            },
            | Err(primary_error) => {
                warn!(
                    error = %primary_error,
                    error_category = primary_error.category(),
                    "Primary operation failed, attempting fallback"
                );

                match fallback_operation().await {
                    | Ok(result) => {
                        info!("Fallback operation succeeded");
                        Ok(result)
                    },
                    | Err(fallback_error) => {
                        error!(
                            primary_error = %primary_error,
                            fallback_error = %fallback_error,
                            "Both primary and fallback operations failed"
                        );

                        // Return the primary error as it's usually more relevant
                        Err(primary_error)
                    },
                }
            },
        }
    }

    #[allow(dead_code)]
    /// Health check with retry logic
    pub async fn health_check<F, Fut>(&self, check_operation: F) -> Result<bool, ServiceError>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<bool, ServiceError>>,
    {
        debug!("Performing health check with retry");

        match self.retry_with_backoff(check_operation).await {
            | Ok(is_healthy) => {
                if is_healthy {
                    debug!("Health check passed");
                } else {
                    warn!("Health check failed - service is unhealthy");
                }
                Ok(is_healthy)
            },
            | Err(error) => {
                error!(
                    error = %error,
                    error_category = error.category(),
                    "Health check failed with error"
                );
                Ok(false) // Return false instead of error for health checks
            },
        }
    }

    #[allow(dead_code)]
    /// Executes multiple operations concurrently with error aggregation
    pub async fn execute_concurrent<T>(
        &self,
        operations: Vec<Box<dyn Future<Output = Result<T, ServiceError>> + Send + Unpin>>,
    ) -> Vec<Result<T, ServiceError>> {
        debug!(operation_count = operations.len(), "Executing concurrent operations");

        let results = futures_util::future::join_all(operations).await;

        let success_count = results.iter().filter(|r| r.is_ok()).count();
        let error_count = results.len() - success_count;

        info!(
            total_operations = results.len(),
            successful = success_count,
            failed = error_count,
            "Concurrent operations completed"
        );

        results
    }

    #[allow(dead_code)]
    /// Batches operations to avoid overwhelming external services
    pub async fn batch_execute<F, Fut, T>(&self, items: Vec<T>, batch_size: usize, operation: F) -> Vec<Result<T, ServiceError>>
    where
        F: Fn(T) -> Fut + Clone + Send + Sync,
        Fut: Future<Output = Result<T, ServiceError>> + Send,
        T: Clone + Send + 'static,
    {
        debug!(total_items = items.len(), batch_size = batch_size, "Executing batched operations");

        let mut results = Vec::with_capacity(items.len());

        for batch in items.chunks(batch_size) {
            debug!(batch_size = batch.len(), "Processing batch");

            let mut batch_futures = Vec::new();
            for item in batch.iter().cloned() {
                let op = operation.clone();
                batch_futures.push(async move { op(item).await });
            }

            let batch_results = futures_util::future::join_all(batch_futures).await;
            results.extend(batch_results);

            // Add small delay between batches to be respectful to external services
            if batch.len() == batch_size {
                sleep(Duration::from_millis(100)).await;
            }
        }

        let success_count = results.iter().filter(|r| r.is_ok()).count();
        info!(
            total_items = results.len(),
            successful = success_count,
            failed = results.len() - success_count,
            "Batched operations completed"
        );

        results
    }
}

/// Circuit breaker implementation for preventing cascading failures
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CircuitBreaker {
    failure_threshold: usize,
    recovery_timeout: Duration,
    state: Arc<tokio::sync::RwLock<CircuitBreakerState>>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct CircuitBreakerState {
    failures: usize,
    last_failure_time: Option<Instant>,
    state: CircuitState,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: usize, recovery_timeout: Duration) -> Self {
        Self {
            failure_threshold,
            recovery_timeout,
            state: Arc::new(tokio::sync::RwLock::new(CircuitBreakerState {
                failures: 0,
                last_failure_time: None,
                state: CircuitState::Closed,
            })),
        }
    }

    pub async fn execute<F, Fut, T>(&self, operation: F) -> Result<T, ServiceError>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<T, ServiceError>>,
    {
        // Check current state
        let should_execute = {
            let mut state = self.state.write().await;

            match state.state {
                | CircuitState::Closed => true,
                | CircuitState::Open => {
                    if let Some(last_failure) = state.last_failure_time {
                        if last_failure.elapsed() >= self.recovery_timeout {
                            debug!("Circuit breaker transitioning to half-open");
                            state.state = CircuitState::HalfOpen;
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                },
                | CircuitState::HalfOpen => true,
            }
        };

        if !should_execute {
            warn!("Circuit breaker is open, rejecting request");
            return Err(ServiceError::external_api("Circuit breaker is open"));
        }

        // Execute operation
        match operation().await {
            | Ok(result) => {
                // Reset on success
                let mut state = self.state.write().await;
                if state.state == CircuitState::HalfOpen {
                    debug!("Circuit breaker transitioning to closed after successful operation");
                    state.state = CircuitState::Closed;
                }
                state.failures = 0;
                state.last_failure_time = None;
                Ok(result)
            },
            | Err(error) => {
                // Handle failure
                let mut state = self.state.write().await;
                state.failures += 1;
                state.last_failure_time = Some(Instant::now());

                if state.failures >= self.failure_threshold && state.state != CircuitState::Open {
                    warn!(
                        failures = state.failures,
                        threshold = self.failure_threshold,
                        "Circuit breaker opening due to failures"
                    );
                    state.state = CircuitState::Open;
                }

                Err(error)
            },
        }
    }

    pub async fn get_state(&self) -> CircuitState {
        let state = self.state.read().await;
        state.state.clone()
    }

    pub async fn get_failure_count(&self) -> usize {
        let state = self.state.read().await;
        state.failures
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use tokio::time::{Duration, sleep};

    #[tokio::test]
    async fn test_retry_success_on_first_attempt() {
        let resilience = ResilienceService::with_default_config();
        let call_count = Arc::new(AtomicUsize::new(0));
        let call_count_clone = call_count.clone();

        let result = resilience
            .retry_with_backoff(|| {
                let count = call_count_clone.clone();
                async move {
                    count.fetch_add(1, Ordering::SeqCst);
                    Ok::<i32, ServiceError>(42)
                }
            })
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        assert_eq!(call_count.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_retry_success_after_failures() {
        let resilience = ResilienceService::new(ResilienceConfig {
            max_retries: 3,
            base_delay_ms: 10, // Short delay for testing
            ..Default::default()
        });

        let call_count = Arc::new(AtomicUsize::new(0));
        let call_count_clone = call_count.clone();

        let result = resilience
            .retry_with_backoff(|| {
                let count = call_count_clone.clone();
                async move {
                    let current_count = count.fetch_add(1, Ordering::SeqCst);
                    if current_count < 2 {
                        Err(ServiceError::network("Temporary failure"))
                    } else {
                        Ok::<i32, ServiceError>(42)
                    }
                }
            })
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        assert_eq!(call_count.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_retry_non_retryable_error() {
        let resilience = ResilienceService::with_default_config();
        let call_count = Arc::new(AtomicUsize::new(0));
        let call_count_clone = call_count.clone();

        let result = resilience
            .retry_with_backoff(|| {
                let count = call_count_clone.clone();
                async move {
                    count.fetch_add(1, Ordering::SeqCst);
                    Err::<i32, ServiceError>(ServiceError::validation("Non-retryable error"))
                }
            })
            .await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ServiceError::Validation(_)));
        assert_eq!(call_count.load(Ordering::SeqCst), 1); // Should not retry
    }

    #[tokio::test]
    async fn test_retry_exhausted() {
        let resilience = ResilienceService::new(ResilienceConfig {
            max_retries: 2,
            base_delay_ms: 10,
            ..Default::default()
        });

        let call_count = Arc::new(AtomicUsize::new(0));
        let call_count_clone = call_count.clone();

        let result = resilience
            .retry_with_backoff(|| {
                let count = call_count_clone.clone();
                async move {
                    count.fetch_add(1, Ordering::SeqCst);
                    Err::<i32, ServiceError>(ServiceError::network("Always fails"))
                }
            })
            .await;

        assert!(result.is_err());
        assert_eq!(call_count.load(Ordering::SeqCst), 3); // Initial + 2 retries
    }

    #[tokio::test]
    async fn test_timeout_success() {
        let resilience = ResilienceService::new(ResilienceConfig {
            operation_timeout_seconds: 1,
            ..Default::default()
        });

        let result = resilience
            .with_timeout(|| async {
                sleep(Duration::from_millis(100)).await;
                Ok::<i32, ServiceError>(42)
            })
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[tokio::test]
    async fn test_timeout_failure() {
        let resilience = ResilienceService::new(ResilienceConfig {
            operation_timeout_seconds: 1,
            ..Default::default()
        });

        let result = resilience
            .with_timeout(|| async {
                sleep(Duration::from_secs(2)).await;
                Ok::<i32, ServiceError>(42)
            })
            .await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ServiceError::Network(_)));
    }

    #[tokio::test]
    async fn test_fallback_primary_success() {
        let resilience = ResilienceService::with_default_config();

        let result = resilience
            .with_fallback(|| async { Ok::<i32, ServiceError>(42) }, || async { Ok::<i32, ServiceError>(99) })
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42); // Primary result
    }

    #[tokio::test]
    async fn test_fallback_primary_failure() {
        let resilience = ResilienceService::new(ResilienceConfig {
            max_retries: 1,
            base_delay_ms: 10,
            operation_timeout_seconds: 1,
            ..Default::default()
        });

        let result = resilience
            .with_fallback(
                || async { Err::<i32, ServiceError>(ServiceError::network("Primary failed")) },
                || async { Ok::<i32, ServiceError>(99) },
            )
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 99); // Fallback result
    }

    #[tokio::test]
    async fn test_circuit_breaker_closed_state() {
        let circuit_breaker = CircuitBreaker::new(3, Duration::from_secs(1));

        let result = circuit_breaker.execute(|| async { Ok::<i32, ServiceError>(42) }).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        assert_eq!(circuit_breaker.get_state().await, CircuitState::Closed);
    }

    #[tokio::test]
    async fn test_circuit_breaker_opens_after_failures() {
        let circuit_breaker = CircuitBreaker::new(2, Duration::from_secs(1));

        // First failure
        let result1 = circuit_breaker
            .execute(|| async { Err::<i32, ServiceError>(ServiceError::network("Failure 1")) })
            .await;
        assert!(result1.is_err());
        assert_eq!(circuit_breaker.get_state().await, CircuitState::Closed);

        // Second failure - should open circuit
        let result2 = circuit_breaker
            .execute(|| async { Err::<i32, ServiceError>(ServiceError::network("Failure 2")) })
            .await;
        assert!(result2.is_err());
        assert_eq!(circuit_breaker.get_state().await, CircuitState::Open);

        // Third attempt should be rejected
        let result3 = circuit_breaker.execute(|| async { Ok::<i32, ServiceError>(42) }).await;
        assert!(result3.is_err());
        assert!(result3.unwrap_err().to_string().contains("Circuit breaker is open"));
    }

    #[tokio::test]
    async fn test_circuit_breaker_recovery() {
        let circuit_breaker = CircuitBreaker::new(1, Duration::from_millis(100));

        // Cause failure to open circuit
        let result1 = circuit_breaker
            .execute(|| async { Err::<i32, ServiceError>(ServiceError::network("Failure")) })
            .await;
        assert!(result1.is_err());
        assert_eq!(circuit_breaker.get_state().await, CircuitState::Open);

        // Wait for recovery timeout
        sleep(Duration::from_millis(150)).await;

        // Should transition to half-open and allow request
        let result2 = circuit_breaker.execute(|| async { Ok::<i32, ServiceError>(42) }).await;
        assert!(result2.is_ok());
        assert_eq!(circuit_breaker.get_state().await, CircuitState::Closed);
    }

    #[tokio::test]
    async fn test_health_check_success() {
        let resilience = ResilienceService::with_default_config();

        let result = resilience.health_check(|| async { Ok::<bool, ServiceError>(true) }).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }

    #[tokio::test]
    async fn test_health_check_failure_returns_false() {
        let resilience = ResilienceService::new(ResilienceConfig {
            max_retries: 1,
            base_delay_ms: 10,
            ..Default::default()
        });

        let result = resilience
            .health_check(|| async { Err::<bool, ServiceError>(ServiceError::network("Health check failed")) })
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), false); // Should return false, not error
    }

    #[tokio::test]
    async fn test_batch_execute() {
        let resilience = ResilienceService::with_default_config();
        let items = vec![1, 2, 3, 4, 5];

        let results = resilience
            .batch_execute(items, 2, |item| async move {
                if item == 3 {
                    Err(ServiceError::validation("Item 3 failed"))
                } else {
                    Ok(item * 2)
                }
            })
            .await;

        assert_eq!(results.len(), 5);
        assert_eq!(results[0].as_ref().unwrap(), &2);
        assert_eq!(results[1].as_ref().unwrap(), &4);
        assert!(results[2].is_err());
        assert_eq!(results[3].as_ref().unwrap(), &8);
        assert_eq!(results[4].as_ref().unwrap(), &10);
    }

    #[tokio::test]
    async fn test_delay_calculation() {
        let resilience = ResilienceService::new(ResilienceConfig {
            base_delay_ms: 1000,
            max_delay_ms: 10000,
            backoff_multiplier: 2.0,
            use_jitter: false,
            ..Default::default()
        });

        let delay0 = resilience.calculate_delay(0);
        let delay1 = resilience.calculate_delay(1);
        let delay2 = resilience.calculate_delay(2);

        assert_eq!(delay0.as_millis(), 1000);
        assert_eq!(delay1.as_millis(), 2000);
        assert_eq!(delay2.as_millis(), 4000);
    }

    #[tokio::test]
    async fn test_delay_with_max_limit() {
        let resilience = ResilienceService::new(ResilienceConfig {
            base_delay_ms: 1000,
            max_delay_ms: 3000,
            backoff_multiplier: 2.0,
            use_jitter: false,
            ..Default::default()
        });

        let delay5 = resilience.calculate_delay(5); // Would be 32000ms without limit
        assert_eq!(delay5.as_millis(), 3000); // Should be capped at max_delay_ms
    }
}
