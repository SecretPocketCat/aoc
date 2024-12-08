use aoc_client::block_on_input;
use criterion::{criterion_group, criterion_main, Criterion};
use day_5::solution::part_a;
use std::hint::black_box;

fn part_a_benchmark(c: &mut Criterion) {
    let input = block_on_input(5);
    c.bench_function("part_a", |b| b.iter(|| part_a(black_box(&input))));
}

criterion_group!(benches, part_a_benchmark);
criterion_main!(benches);
