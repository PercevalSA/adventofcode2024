use adventofcode2024::{day2, day3, utils};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn bench_day_2(c: &mut Criterion) {
    c.bench_function("day 2 part 1", |b| b.iter(|| day2::part1()));
    c.bench_function("day 2 part 2", |b| b.iter(|| day2::part2()));
}

pub fn bench_day_3(c: &mut Criterion) {
    let data_str = utils::read_file("3");
    let data = data_str.as_str();

    c.bench_function("day 3 part 1", |b| b.iter(|| day3::part1(black_box(data))));
}

criterion_group!(benches, bench_day_2, bench_day_3,);
criterion_main!(benches);
