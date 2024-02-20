use benchmark::factorial;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("tail_recursion", |b| {
        b.iter(|| {
            for _ in 0..999_999 {
                let res = factorial(15, 1);
                black_box(res);
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
