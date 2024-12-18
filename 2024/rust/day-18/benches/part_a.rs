use aoc_client::block_on_input;
use criterion::{criterion_group, criterion_main, Criterion};
use day_18::solution::part_a;
use std::hint::black_box;

fn part_a_benchmark(c: &mut Criterion) {
    let input = block_on_input(18);
    let mut group = c.benchmark_group("day-18");
    // group
    //     .sample_size(10)
    //     .measurement_time(std::time::Duration::from_secs(30));
    group.bench_function("part_a", |b| b.iter(|| part_a(black_box(&input))));
    group.finish();
}

criterion_group!(benches, part_a_benchmark);
criterion_main!(benches);
