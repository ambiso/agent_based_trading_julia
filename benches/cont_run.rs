// resides in benches/cont_run.rs
//

use agent_based_trading_julia::{cont_run, FRNG, dSFMT};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::SmallRng;
use rand::{thread_rng, Rng, SeedableRng};
use xorshift::{Rand, SplitMix64, Xoroshiro128, Xorshift128, Xorshift1024};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut v = vec![0.0; 10_000];
    let mut uni = SmallRng::from_rng(thread_rng())
        .unwrap()
        .sample_iter(rand::distributions::Uniform::new_inclusive(0., 1.));

    // Manually seed a Xorshift128+ PRNG
    let mut xorshift: Xorshift128 = xorshift::SeedableRng::from_seed(&[0xdeadbeef, 0xdeadbeef][..]);
    {
        use xorshift::Rng;
        c.bench_function("xorshift", |b| {
            b.iter(|| {
                v.iter_mut()
                    .for_each(|x| *x = xorshift.next_f64());
            })
        });
    }
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
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
