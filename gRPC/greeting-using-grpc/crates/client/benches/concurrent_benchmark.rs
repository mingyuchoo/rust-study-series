use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use greeting_client::{connect_client, create_and_send_request, process_response};
use std::hint::black_box;
use tokio::runtime::Runtime;

fn concurrent_requests_benchmark(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    let mut group = c.benchmark_group("concurrent_requests");
    
    // Benchmark different levels of concurrency
    for concurrency in [1, 5, 10, 20].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(concurrency),
            concurrency,
            |b, &concurrency| {
                b.iter(|| {
                    rt.block_on(async {
                        let client = connect_client().await.unwrap();
                        
                        let mut handles = Vec::new();
                        
                        for i in 0..concurrency {
                            let client_clone = client.clone();
                            let handle = tokio::spawn(async move {
                                let name = black_box(format!("User{}", i));
                                let response = create_and_send_request(client_clone, &name)
                                    .await
                                    .unwrap();
                                let _message = process_response(response).await.unwrap();
                            });
                            handles.push(handle);
                        }
                        
                        for handle in handles {
                            handle.await.unwrap();
                        }
                    });
                });
            },
        );
    }
    
    group.finish();
}

fn connection_overhead_benchmark(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    c.bench_function("connection_establishment", |b| {
        b.iter(|| {
            rt.block_on(async {
                let _client = connect_client().await.unwrap();
            });
        })
    });
}

criterion_group!(benches, concurrent_requests_benchmark, connection_overhead_benchmark);
criterion_main!(benches);
