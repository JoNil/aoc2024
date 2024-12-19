use aoc2024::day19;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day19::b", |b| {
        b.iter(|| day19::b(day19::INPUT));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
