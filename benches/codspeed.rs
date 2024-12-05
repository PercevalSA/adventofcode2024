use adventofcode2024::{day2, day3, day4, utils};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn bench_day_2(c: &mut Criterion) {
    let data_str = utils::read_file("day2");
    let data = day2::parse_data(data_str.as_str());

    c.bench_function("day 2 part 1", |b| b.iter(|| day2::part1(&data)));
    c.bench_function("day 2 part 2", |b| b.iter(|| day2::part2(&data)));
}

pub fn bench_day_3(c: &mut Criterion) {
    let data_str = utils::read_file("day3");
    let data = data_str.as_str();

    c.bench_function("day 3 part 1", |b| b.iter(|| day3::part1(black_box(data))));
    c.bench_function("day 3 part 2", |b| b.iter(|| day3::part2(black_box(data))));
}

pub fn bench_day_4(c: &mut Criterion) {
    let data_str = utils::read_file("day4");
    let data = day4::parse_input(data_str.as_str());

    c.bench_function("day 4 part 1", |b| b.iter(|| day4::part1(black_box(&data))));
    // c.bench_function("day 4 part 2", |b| b.iter(|| day4::part2(black_box(data))));
}

criterion_group!(benches, bench_day_2, bench_day_3);
criterion_main!(benches);
