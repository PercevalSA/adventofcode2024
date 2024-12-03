use adventofcode2024::day_2::solve_one;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

pub fn criterion_bench_day_two_first(c: &mut Criterion) {
    c.bench_function("day 2 first", |b| b.iter(|| black_box(solve_one())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

// fn bench_two(bench: &mut Bencher) {
//     bench.iter(|| {
//         day_2::solve_two();
//     })
// }
