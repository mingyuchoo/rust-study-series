use criterion::{black_box, criterion_group, criterion_main, Criterion};
use greeting_using_rpc::client_service::{connect_client, create_and_send_request, process_response};
use greeting_using_rpc::error::AppResult;
use tokio::runtime::Runtime;

fn client_request_benchmark(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    // First ensure we have a connection
    let client = rt.block_on(async { connect_client().await.unwrap() });
    
    c.bench_function("grpc_client_request", |b| {
        b.iter(|| {
            rt.block_on(async {
                let name = black_box("Benchmark User");
                let response = create_and_send_request(client.clone(), name).await.unwrap();
                let _message = process_response(response).await.unwrap();
            });
        })
    });
}

criterion_group!(benches, client_request_benchmark);
criterion_main!(benches);
