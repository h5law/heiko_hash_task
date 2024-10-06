use criterion::async_executor::FuturesExecutor;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use heiko_hash_task::{
    aggregator::{
        aggregate_hashes, aggregate_hashes_async_parts, aggregate_hashes_halves,
        aggregate_hashes_parts,
    },
    helpers::generate_random_hashes,
};

fn benchmark_aggregate_hashes(c: &mut Criterion) {
    let hashes = generate_random_hashes();

    c.bench_function("aggreagete hashes", |b| {
        b.iter(|| aggregate_hashes(black_box(&hashes)))
    });
}

fn benchmark_aggregate_hashes_parts(c: &mut Criterion) {
    let hashes = generate_random_hashes();

    c.bench_function("aggregate hashes parts", |b| {
        b.iter(|| aggregate_hashes_parts(black_box(&hashes)))
    });
}

fn benchmark_aggregate_hashes_halves(c: &mut Criterion) {
    let hashes = generate_random_hashes();

    c.bench_function("aggregate hashes halves", |b| {
        b.iter(|| aggregate_hashes_halves(black_box(&hashes)))
    });
}

fn benchmark_aggregate_hashes_parts_async(c: &mut Criterion) {
    let hashes = generate_random_hashes();

    c.bench_function("aggregate hashes async parts", |b| {
        b.to_async(FuturesExecutor)
            .iter(|| aggregate_hashes_async_parts(black_box(&hashes)))
    });
}

criterion_group!(
    benches,
    benchmark_aggregate_hashes,
    benchmark_aggregate_hashes_parts,
    benchmark_aggregate_hashes_halves,
    benchmark_aggregate_hashes_parts_async
);
criterion_main!(benches);
