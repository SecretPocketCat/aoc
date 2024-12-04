use bench_all::{get_inputs, run_all};
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn bench_all(c: &mut Criterion) {
    let inputs = get_inputs();
    c.bench_function("bench_all", |b| b.iter(|| run_all(black_box(&inputs))));
}

criterion_group!(benches, bench_all);
criterion_main!(benches);
