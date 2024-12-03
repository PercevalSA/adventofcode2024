use adventofcode2024::day2::part1;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_bench_day_two_first(c: &mut Criterion) {
    c.bench_function("day 2 first", |b| b.iter(|| black_box(part1())));
}

criterion_group!(benches, criterion_bench_day_two_first);
criterion_main!(benches);

// fn bench_two(bench: &mut Bencher) {
//     bench.iter(|| {
//         day_2::solve_two();
//     })
// }
