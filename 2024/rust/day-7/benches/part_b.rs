use aoc_client::block_on_input;
use criterion::{criterion_group, criterion_main, Criterion};
use day_7::solution::part_b;
use std::hint::black_box;

fn part_b_benchmark(c: &mut Criterion) {
    let input = block_on_input(7);
    c.bench_function("part_b", |b| b.iter(|| part_b(black_box(&input))));
}

criterion_group!(benches, part_b_benchmark);
criterion_main!(benches);
