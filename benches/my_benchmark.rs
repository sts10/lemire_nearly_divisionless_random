// lot of copy and pasting from:
// https://bheisler.github.io/criterion.rs/book/getting_started.html
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lemire::roll_using_gen_range;
use lemire::roll_using_lemire_fast;
use lemire::roll_using_readable_lemire;
use lemire::roll_using_traditional_rejection_method;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("'lemire fast', s = 6", |b| {
        b.iter(|| roll_using_lemire_fast(black_box(6)))
    });

    c.bench_function("Lemire readable, s = 6", |b| {
        b.iter(|| roll_using_readable_lemire(black_box(6)))
    });

    c.bench_function("Rand crate, s = 6", |b| {
        b.iter(|| roll_using_gen_range(black_box(6)))
    });

    c.bench_function("Traditional rejection method, s=6", |b| {
        b.iter(|| roll_using_traditional_rejection_method(black_box(6)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
