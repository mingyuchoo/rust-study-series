use criterion::{Criterion,
                criterion_group,
                criterion_main};
use greeting_server::process_greeting_request;
use std::hint::black_box;

fn greeting_processing_benchmark(c: &mut Criterion) {
    c.bench_function("process_greeting_request", |b| {
        b.iter(|| {
            let name = black_box("Benchmark User".to_string());
            let _response = process_greeting_request(name).unwrap();
        })
    });
}

fn greeting_multiple_names_benchmark(c: &mut Criterion) {
    let names = vec!["Alice", "Bob", "Charlie", "David", "Eve"];

    c.bench_function("process_multiple_greetings", |b| {
        b.iter(|| {
            for name in &names {
                let _response = process_greeting_request(black_box(name.to_string())).unwrap();
            }
        })
    });
}

criterion_group!(benches, greeting_processing_benchmark, greeting_multiple_names_benchmark);
criterion_main!(benches);
