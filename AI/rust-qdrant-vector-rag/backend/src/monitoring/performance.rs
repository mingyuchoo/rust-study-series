use crate::monitoring::Metrics;
use std::sync::Arc;
use std::time::Duration;
use sysinfo::{Pid, System};
use tokio::sync::RwLock;
use tokio::time::interval;
use tracing::{debug, error, info, warn};

/// Performance monitor that tracks system resources and application metrics
pub struct PerformanceMonitor {
    system: Arc<RwLock<System>>,
    is_running: Arc<RwLock<bool>>,
}

impl PerformanceMonitor {
    /// Create a new performance monitor
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();

        Self {
            system: Arc::new(RwLock::new(system)),
            is_running: Arc::new(RwLock::new(false)),
        }
    }

    /// Start the performance monitoring background task
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut is_running = self.is_running.write().await;
        if *is_running {
            warn!("Performance monitor is already running");
            return Ok(());
        }

        *is_running = true;
        drop(is_running);

        info!("Starting performance monitor...");

        let system = Arc::clone(&self.system);
        let is_running = Arc::clone(&self.is_running);

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(30)); // Update every 30 seconds

            loop {
                interval.tick().await;

                let running = *is_running.read().await;
                if !running {
                    info!("Performance monitor stopped");
                    break;
                }

                if let Err(e) = Self::collect_system_metrics(&system).await {
                    error!("Failed to collect system metrics: {}", e);
                }
            }
        });

        info!("Performance monitor started successfully");
        Ok(())
    }

    /// Stop the performance monitoring
    pub async fn stop(&self) {
        let mut is_running = self.is_running.write().await;
        *is_running = false;
        info!("Performance monitor stop requested");
    }

    /// Collect and record system metrics
    async fn collect_system_metrics(system: &Arc<RwLock<System>>) -> Result<(), Box<dyn std::error::Error>> {
        let mut sys = system.write().await;

        // Refresh system information
        sys.refresh_cpu();
        sys.refresh_memory();
        sys.refresh_processes();

        // Get current process info
        let current_pid = Pid::from(std::process::id() as usize);

        if let Some(process) = sys.process(current_pid) {
            // Record memory usage
            let memory_bytes = process.memory();
            Metrics::set_memory_usage(memory_bytes);
            debug!("Memory usage: {} MB", memory_bytes / 1024 / 1024);

            // Record CPU usage (average across all cores)
            let cpu_usage = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / sys.cpus().len() as f32;
            Metrics::set_cpu_usage(cpu_usage as f64);
            debug!("CPU usage: {:.2}%", cpu_usage);
        }

        // Record total system memory
        let total_memory = sys.total_memory();
        let used_memory = sys.used_memory();
        let memory_usage_percent = (used_memory as f64 / total_memory as f64) * 100.0;

        debug!(
            "System memory: {:.2}% used ({} MB / {} MB)",
            memory_usage_percent,
            used_memory / 1024 / 1024,
            total_memory / 1024 / 1024
        );

        Ok(())
    }

    /// Get current system performance snapshot
    pub async fn get_performance_snapshot(&self) -> Result<PerformanceSnapshot, Box<dyn std::error::Error>> {
        let mut sys = self.system.write().await;
        sys.refresh_all();

        let current_pid = Pid::from(std::process::id() as usize);

        let process_info = sys.process(current_pid).map(|process| ProcessInfo {
            memory_bytes: process.memory(),
            cpu_usage: process.cpu_usage(),
        });

        let cpu_usage = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / sys.cpus().len() as f32;

        Ok(PerformanceSnapshot {
            timestamp: chrono::Utc::now(),
            system_memory_total: sys.total_memory(),
            system_memory_used: sys.used_memory(),
            system_cpu_usage: cpu_usage,
            process_info,
        })
    }
}

impl Default for PerformanceMonitor {
    fn default() -> Self { Self::new() }
}

/// Snapshot of current performance metrics
#[derive(Debug, Clone)]
pub struct PerformanceSnapshot {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub system_memory_total: u64,
    pub system_memory_used: u64,
    pub system_cpu_usage: f32,
    pub process_info: Option<ProcessInfo>,
}

/// Information about the current process
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub memory_bytes: u64,
    pub cpu_usage: f32,
}

impl PerformanceSnapshot {
    /// Get memory usage percentage
    pub fn memory_usage_percent(&self) -> f64 { (self.system_memory_used as f64 / self.system_memory_total as f64) * 100.0 }

    /// Get process memory in MB
    pub fn process_memory_mb(&self) -> Option<u64> { self.process_info.as_ref().map(|info| info.memory_bytes / 1024 / 1024) }

    /// Check if system is under memory pressure
    pub fn is_memory_pressure(&self) -> bool { self.memory_usage_percent() > 85.0 }

    /// Check if system is under CPU pressure
    pub fn is_cpu_pressure(&self) -> bool { self.system_cpu_usage > 80.0 }
}

/// Performance timing utility for measuring operation durations
pub struct PerformanceTimer {
    start_time: instant::Instant,
    operation_name: String,
}

impl PerformanceTimer {
    /// Start timing an operation
    pub fn start(operation_name: impl Into<String>) -> Self {
        Self {
            start_time: instant::Instant::now(),
            operation_name: operation_name.into(),
        }
    }

    /// Finish timing and record the duration
    pub fn finish(self) -> Duration {
        let duration = self.start_time.elapsed();
        debug!("Operation '{}' completed in {:?}", self.operation_name, duration);
        duration
    }

    /// Finish timing and record to specific metric
    pub fn finish_with_metric<F>(self, record_fn: F) -> Duration
    where
        F: FnOnce(Duration),
    {
        let duration = self.finish();
        record_fn(duration);
        duration
    }
}

/// Memory usage optimizer for large document processing
pub struct MemoryOptimizer {
    max_chunk_size: usize,
    max_concurrent_operations: usize,
}

impl MemoryOptimizer {
    /// Create a new memory optimizer with default settings
    pub fn new() -> Self {
        Self {
            max_chunk_size: 1024 * 1024, // 1MB chunks
            max_concurrent_operations: num_cpus::get() * 2,
        }
    }

    /// Create with custom settings
    pub fn with_settings(max_chunk_size: usize, max_concurrent_operations: usize) -> Self {
        Self {
            max_chunk_size,
            max_concurrent_operations,
        }
    }

    /// Process large content in memory-efficient chunks
    pub async fn process_large_content<T, F, Fut>(&self, content: &str, processor: F) -> Result<Vec<T>, Box<dyn std::error::Error>>
    where
        F: Fn(String) -> Fut + Send + Sync,
        Fut: std::future::Future<Output = Result<T, Box<dyn std::error::Error>>> + Send,
        T: Send,
    {
        let chunks = self.split_content(content);
        let mut results = Vec::with_capacity(chunks.len());

        // Process chunks in batches to limit memory usage
        for batch in chunks.chunks(self.max_concurrent_operations) {
            let batch_futures: Vec<_> = batch.iter().map(|chunk| processor(chunk.clone())).collect();

            let batch_results = futures::future::try_join_all(batch_futures).await?;
            results.extend(batch_results);
        }

        Ok(results)
    }

    /// Split content into memory-efficient chunks
    fn split_content(&self, content: &str) -> Vec<String> {
        if content.len() <= self.max_chunk_size {
            return vec![content.to_string()];
        }

        let mut chunks = Vec::new();
        let mut start = 0;

        while start < content.len() {
            let end = std::cmp::min(start + self.max_chunk_size, content.len());

            // Try to find a good break point (word boundary)
            let chunk_end = if end < content.len() {
                content[start .. end].rfind(|c: char| c.is_whitespace()).map(|pos| start + pos).unwrap_or(end)
            } else {
                end
            };

            chunks.push(content[start .. chunk_end].to_string());
            start = chunk_end;

            // Skip whitespace at the beginning of the next chunk
            while start < content.len() && content.chars().nth(start).unwrap_or('\0').is_whitespace() {
                start += 1;
            }
        }

        chunks
    }

    /// Get recommended batch size based on available memory
    pub async fn get_recommended_batch_size(&self, item_size_estimate: usize) -> usize {
        // Get current memory usage
        let mut system = System::new();
        system.refresh_memory();

        let available_memory = system.total_memory() - system.used_memory();
        let safe_memory = (available_memory as f64 * 0.1) as u64; // Use only 10% of available memory

        let max_items = (safe_memory / item_size_estimate as u64) as usize;
        std::cmp::max(1, std::cmp::min(max_items, 100)) // Between 1 and 100 items
    }
}

impl Default for MemoryOptimizer {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_optimizer_content_splitting() {
        let optimizer = MemoryOptimizer::with_settings(10, 2); // Small chunk size for testing
        let content = "This is a test content that should be split into multiple chunks";

        let chunks = optimizer.split_content(content);

        assert!(chunks.len() > 1, "Content should be split into multiple chunks");

        // Verify all chunks are within size limit (allowing for word boundaries)
        for chunk in &chunks {
            assert!(chunk.len() <= 15, "Chunk size should be reasonable: '{}'", chunk); // Allow some flexibility for word boundaries
        }

        // Verify content is preserved when joined
        let rejoined = chunks.join(" ");
        assert!(rejoined.contains("This is a test"), "Content should be preserved");
    }

    #[test]
    fn test_performance_timer() {
        let timer = PerformanceTimer::start("test_operation");
        std::thread::sleep(Duration::from_millis(10));
        let duration = timer.finish();

        assert!(duration >= Duration::from_millis(10), "Timer should measure at least the sleep duration");
    }

    #[tokio::test]
    async fn test_performance_monitor_creation() {
        let monitor = PerformanceMonitor::new();
        let snapshot = monitor.get_performance_snapshot().await;

        assert!(snapshot.is_ok(), "Should be able to get performance snapshot");

        let snapshot = snapshot.unwrap();
        assert!(snapshot.system_memory_total > 0, "Should have system memory info");
    }
}
