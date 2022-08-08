use criterion::{black_box, criterion_group, criterion_main, Criterion};
use functional_features::iterators::{create_vec, sum, sum_generic};

fn bench_sum(c: &mut Criterion) {
    let vec = create_vec(black_box(100));
    c.bench_function("sum vec 100", |b| b.iter(|| sum(&vec)));
}

fn bench_sum_generic(c: &mut Criterion) {
    let vec = create_vec(black_box(100));
    c.bench_function("sum_generic vec 100", |b| {
        b.iter(|| sum_generic(&vec));
    });
}

criterion_group!(benches, bench_sum, bench_sum_generic);
criterion_main!(benches);
