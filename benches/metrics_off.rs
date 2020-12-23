use criterion::{criterion_group, criterion_main, Criterion};

mod metrics_bench_util;

fn benchmark(c: &mut Criterion) {
    metrics_bench_util::benchmark(c, false)
}

criterion_group!(benches, benchmark);
criterion_main!(benches);