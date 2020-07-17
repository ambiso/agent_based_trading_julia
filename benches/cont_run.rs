// resides in benches/cont_run.rs
//

use agent_based_trading_julia::{cont_run, FRNG, dSFMT};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::SmallRng;
use rand::{thread_rng, Rng, SeedableRng};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("cont_run short", |b| {
        b.iter(|| {
            black_box(cont_run(
                black_box(1_000),
                black_box(1_000),
                black_box(0.05),
                black_box(0.1),
            ))
        })
    });
    c.bench_function("cont_run long", |b| {
        b.iter(|| {
            black_box(cont_run(
                black_box(10_000),
                black_box(10_000),
                black_box(0.05),
                black_box(0.1),
            ))
        })
    });
    let mut v = vec![0.0; 10_000];
    let mut uni = SmallRng::from_rng(thread_rng())
        .unwrap()
        .sample_iter(rand::distributions::Uniform::new_inclusive(0., 1.));
    c.bench_function("random_numbers", |b| {
        b.iter(|| {
            v.iter_mut()
                .for_each(|x| *x = uni.next().unwrap());
        })
    });
    let mut trash_rng = FRNG::new();
    c.bench_function("trash random numbers", |b| {
        b.iter(|| {
            trash_rng.fill(&mut v);
        })
    });
    let mut fancy = dSFMT::new();
    c.bench_function("fancy random numbers", |b| {
        b.iter(|| {
            fancy.fill(&mut v);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
