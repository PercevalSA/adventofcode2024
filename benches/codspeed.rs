use adventofcode2024::{day2, day3};
use criterion::{criterion_group, criterion_main, Criterion};
use std::fs::File;
use std::io::Read;

pub fn bench_day_2_part_1(c: &mut Criterion) {
    c.bench_function("day 2 part 1", |b| b.iter(|| day2::part1()));
}

pub fn bench_day_2_part_2(c: &mut Criterion) {
    c.bench_function("day 2 part 2", |b| b.iter(|| day2::part2()));
}

pub fn bench_day_3_part_1(c: &mut Criterion) {
    let mut file = File::open("../resources/3.txt").expect("can't read file");
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    let data = content.as_str();

    c.bench_function("day 3 part 1", |b| b.iter(|| day3::part1(data)));
}

criterion_group!(
    benches,
    bench_day_2_part_1,
    bench_day_2_part_2,
    bench_day_3_part_1
);
criterion_main!(benches);
