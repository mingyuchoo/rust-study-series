use criterion::{Criterion,
                criterion_group,
                criterion_main};
use greeting_client::{connect_client,
                      create_and_send_request,
                      process_response};
use std::hint::black_box;
use tokio::runtime::Runtime;

fn client_request_benchmark(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    // Establish the connection once outside the benchmark loop
    let client = rt.block_on(async { connect_client().await.unwrap() });

    c.bench_function("grpc_client_request", |b| {
        b.iter(|| {
            rt.block_on(async {
                let name = black_box("Benchmark User");
                let response = create_and_send_request(client.clone(), name).await.unwrap();
                let _message = process_response(response).unwrap();
            });
        })
    });
}

criterion_group!(benches, client_request_benchmark);
criterion_main!(benches);
