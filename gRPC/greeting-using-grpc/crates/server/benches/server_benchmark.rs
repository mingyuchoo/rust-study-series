use criterion::{criterion_group, criterion_main, Criterion};
use greeting_server::process_greeting_request;
use std::hint::black_box;
use tokio::runtime::Runtime;

fn greeting_processing_benchmark(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    c.bench_function("process_greeting_request", |b| {
        b.iter(|| {
            rt.block_on(async {
                let name = black_box("Benchmark User".to_string());
                let _response = process_greeting_request(name).await.unwrap();
            });
        })
    });
}

fn greeting_multiple_names_benchmark(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let names = vec!["Alice", "Bob", "Charlie", "David", "Eve"];
    
    c.bench_function("process_multiple_greetings", |b| {
        b.iter(|| {
            rt.block_on(async {
                for name in &names {
                    let _response = process_greeting_request(black_box(name.to_string())).await.unwrap();
                }
            });
        })
    });
}

criterion_group!(benches, greeting_processing_benchmark, greeting_multiple_names_benchmark);
criterion_main!(benches);
