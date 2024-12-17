use aoc_client::block_on_input;
use criterion::{criterion_group, criterion_main, Criterion};
use day_14::solution::part_b;
use std::hint::black_box;

fn part_b_benchmark(c: &mut Criterion) {
    let input = block_on_input(14);
    let mut group = c.benchmark_group("day-14");
    // group
    //     .sample_size(10)
    //     .measurement_time(std::time::Duration::from_secs(30));
    group.bench_function("part_b", |b| b.iter(|| part_b(black_box(&input))));
    group.finish();
}

criterion_group!(benches, part_b_benchmark);
criterion_main!(benches);
