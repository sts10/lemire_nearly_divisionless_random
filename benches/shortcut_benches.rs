use criterion::{criterion_group, criterion_main, Criterion};

extern crate rand;
use rand::distributions::{Distribution, Uniform};

pub fn shortcut1(c: &mut Criterion) {
    // make a random possible value for s
    let between = Uniform::from(0..u8::MAX);
    let mut rng = rand::thread_rng();
    let random_s = between.sample(&mut rng);

    let mut group = c.benchmark_group("(256 % s)");
    // Not sure if this cast `as u16` is fair to the traditional method here
    group.bench_function("traditional", |b| b.iter(|| 256 % (random_s as u16)));

    group.bench_function("shortcut", |b| {
        b.iter(|| (u8::MAX - (random_s + 1) % random_s))
    });
}

pub fn shortcut2(c: &mut Criterion) {
    // make a random possible value for m
    let between = Uniform::from(0..u16::MAX);
    let mut rng = rand::thread_rng();
    let random_m = between.sample(&mut rng);

    let mut group = c.benchmark_group("(m / 256)");
    group.bench_function("traditional", |b| b.iter(|| (random_m / 256)));
    group.bench_function("shortcut", |b| b.iter(|| (random_m >> 8)));
}

pub fn shortcut3(c: &mut Criterion) {
    // make a random possible value for m
    let between = Uniform::from(0..u16::MAX);
    let mut rng = rand::thread_rng();
    let random_m = between.sample(&mut rng);

    let mut group = c.benchmark_group("(m % 256)");
    group.bench_function("traditional", |b| b.iter(|| (random_m % 256)));
    group.bench_function("shortcut", |b| b.iter(|| (random_m as u8)));
}

criterion_group!(benches, shortcut1, shortcut2, shortcut3);
criterion_main!(benches);
