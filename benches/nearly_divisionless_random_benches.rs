// lot of copy and pasting from:
// https://bheisler.github.io/criterion.rs/book/getting_started.html
extern crate rand;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lemire::roll_using_lemire_fast;
use lemire::roll_using_traditional_rejection_method;
use rand::distributions::{Distribution, Uniform};

#[path = "../src/readable.rs"]
pub mod readable;
use readable::roll_using_readable_lemire;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Roll die");
    group.bench_function("'Lemire fast'", |b| {
        b.iter(|| roll_using_lemire_fast(black_box(6)))
    });

    group.bench_function("Lemire readable", |b| {
        b.iter(|| roll_using_readable_lemire(black_box(6)))
    });

    group.bench_function("Rand crate", |b| {
        let between = Uniform::from(0..6);
        let mut rng = rand::thread_rng();
        b.iter(|| between.sample(&mut rng));
    });

    group.bench_function("Traditional rejection method", |b| {
        b.iter(|| roll_using_traditional_rejection_method(black_box(6)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
