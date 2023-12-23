
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pi_calc::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("pi_fold_inline 100000", |b| b.iter(|| calc_pi_fold_inline(black_box(100000))));
    c.bench_function("pi_fold 100000", |b| b.iter(|| calc_pi_fold(black_box(100000))));
    c.bench_function("pi_foreach_inline 100000", |b| b.iter(|| calc_pi_foreach_inline(black_box(100000))));
    c.bench_function("pi_foreach 100000", |b| b.iter(|| calc_pi_foreach_inline(black_box(100000))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);