use criterion::{criterion_group, criterion_main, Criterion};
use day_8::Forest;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut forest = Forest::new("../inputs/day_8.txt");
    c.bench_function("single-threaded", 
        |b| b.iter(|| forest.calc_visibility())
    );

    let mut forest = Forest::new("../inputs/day_8.txt");
    c.bench_function("multi-threaded", 
        |b| b.iter(|| forest.calc_multi())
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);