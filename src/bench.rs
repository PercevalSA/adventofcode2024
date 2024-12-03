extern crate bencher;

use bencher::{benchmark_group, benchmark_main, Bencher};
#[path = "day_2.rs"]
mod day_2;

fn bench_one(bench: &mut Bencher) {
    bench.iter(|| {
        day_2::solve_one();
    })
}

fn bench_two(bench: &mut Bencher) {
    bench.iter(|| {
        day_2::solve_two();
    })
}


benchmark_group!(benches, bench_one, bench_two);
benchmark_main!(benches);