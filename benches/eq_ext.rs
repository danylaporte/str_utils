use criterion::{black_box, criterion_group, criterion_main, Criterion};
use str_utils::cmp::EqExt;

fn eq_ci(s: &str) -> bool {
    s.eq_ci("AeC")
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("eq_ci", |b| b.iter(|| eq_ci(black_box("aec"))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
