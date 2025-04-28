use lazy_static::lazy_static;
use prometheus::{Counter, Gauge, Histogram, Registry, opts};
use std::time::Instant;

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
    
    // Counter for total number of requests processed
    pub static ref REQUESTS_TOTAL: Counter = Counter::with_opts(opts!(
        "llm_requests_total",
        "Total number of LLM requests processed"
    )).unwrap();
    
    // Counter for failed requests
    pub static ref REQUESTS_FAILED: Counter = Counter::with_opts(opts!(
        "llm_requests_failed",
        "Total number of failed LLM requests"
    )).unwrap();
    
    // Histogram for request duration
    pub static ref REQUEST_DURATION: Histogram = Histogram::with_opts(
        prometheus::HistogramOpts::new(
            "llm_request_duration_seconds",
            "Duration of LLM requests in seconds"
        ).buckets(vec![0.01, 0.1, 0.5, 1.0, 2.0, 5.0, 10.0])
    ).unwrap();
    
    // Gauge for active agents count
    pub static ref ACTIVE_AGENTS: Gauge = Gauge::with_opts(opts!(
        "llm_active_agents",
        "Number of active LLM agents"
    )).unwrap();
}

pub fn register_metrics() {
    REGISTRY.register(Box::new(REQUESTS_TOTAL.clone())).unwrap();
    REGISTRY.register(Box::new(REQUESTS_FAILED.clone())).unwrap();
    REGISTRY.register(Box::new(REQUEST_DURATION.clone())).unwrap();
    REGISTRY.register(Box::new(ACTIVE_AGENTS.clone())).unwrap();
}

pub struct RequestTimer {
    start_time: Instant,
}

impl RequestTimer {
    pub fn new() -> Self {
        REQUESTS_TOTAL.inc();
        Self {
            start_time: Instant::now(),
        }
    }
    
    pub fn observe_success(self) {
        let duration = self.start_time.elapsed().as_secs_f64();
        REQUEST_DURATION.observe(duration);
    }
    
    pub fn observe_failure(self) {
        REQUESTS_FAILED.inc();
        let duration = self.start_time.elapsed().as_secs_f64();
        REQUEST_DURATION.observe(duration);
    }
}

pub fn increment_active_agents() {
    ACTIVE_AGENTS.inc();
}

pub fn decrement_active_agents() {
    ACTIVE_AGENTS.dec();
}

pub fn set_active_agents(count: i64) {
    ACTIVE_AGENTS.set(count);
}

pub fn start_metrics_server(addr: &str) -> std::io::Result<()> {
    use prometheus::Encoder;
    use std::net::SocketAddr;
    use std::str::FromStr;
    use tokio::net::TcpListener;
    use tokio::io::AsyncWriteExt;
    
    let addr = SocketAddr::from_str(addr).unwrap();
    
    tokio::spawn(async move {
        let listener = TcpListener::bind(addr).await.unwrap();
        println!("Metrics server listening on {}", addr);
        
        loop {
            match listener.accept().await {
                Ok((mut stream, _)) => {
                    let encoder = prometheus::TextEncoder::new();
                    let metrics = REGISTRY.gather();
                    let mut buffer = Vec::new();
                    
                    if let Err(e) = encoder.encode(&metrics, &mut buffer) {
                        eprintln!("Failed to encode metrics: {}", e);
                        continue;
                    }
                    
                    if let Err(e) = stream.write_all(&buffer).await {
                        eprintln!("Failed to write metrics to client: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to accept connection: {}", e);
                }
            }
        }
    });
    
    Ok(())
}
