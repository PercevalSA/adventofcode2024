use adventofcode2024::{day2, day3};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn bench_day_2_part_1(c: &mut Criterion) {
    c.bench_function("day 2 part 1", |b| b.iter(|| day2::part1()));
}

pub fn bench_day_2_part_2(c: &mut Criterion) {
    c.bench_function("day 2 part 2", |b| b.iter(|| day2::part2()));
}

pub fn bench_day_3_part_1(c: &mut Criterion) {
    c.bench_function("day 3 part 1", |b| b.iter(|| day3::part1()));
}

criterion_group!(benches, bench_day_2_part_1, bench_day_2_part_2);
criterion_main!(benches);
