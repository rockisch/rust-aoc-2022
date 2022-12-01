
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc_rust::day01;

pub fn criterion_benchmark(c: &mut Criterion) {
    let bench_data = include_str!("../inputs/day01_e.txt");
    c.bench_function("day 1", |b| b.iter(|| day01::solve_str::<3>(black_box(bench_data))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
