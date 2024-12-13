use aoc2024::days;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day 1", |b| b.iter(|| days::day01::solve()));
    c.bench_function("day 2", |b| b.iter(|| days::day02::solve()));
    c.bench_function("day 3", |b| b.iter(|| days::day03::solve()));
    c.bench_function("day 4", |b| b.iter(|| days::day04::solve()));
    c.bench_function("day 5", |b| b.iter(|| days::day05::solve()));
    c.bench_function("day 6", |b| b.iter(|| days::day06::solve()));
    c.bench_function("day 7", |b| b.iter(|| days::day07::solve()));
    c.bench_function("day 8", |b| b.iter(|| days::day08::solve()));
    c.bench_function("day 12", |b| b.iter(|| days::day12::solve()));
    c.bench_function("day 13", |b| b.iter(|| days::day13::solve()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
