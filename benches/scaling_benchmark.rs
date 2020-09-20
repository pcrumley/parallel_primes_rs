use criterion::{black_box, criterion_group, criterion_main, Criterion};
use primes_lib::primes;

def 

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("one core", |b| b.iter(|| primes(black_box(2000), black_box(5000))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
