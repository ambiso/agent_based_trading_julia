// resides in benches/cont_run.rs
//

use agent_based_trading_julia::{cont_run, dSFMT, fill_f64};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use packed_simd::f64x4;
use rand::prelude::SmallRng;
use rand::{thread_rng, Rng};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut v = vec![0.0; 10_000];

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
    {
        use rand::distributions::Distribution;
        use simd_prngs::rng_impl::SeedableRng;
        use simd_prngs::Xorshift128PlusX4;
        let mut rng =
            Xorshift128PlusX4::from_rng(simd_prngs::rng_impl::rand::thread_rng()).unwrap();
        let dist = rand::distributions::Open01;
        c.bench_function("Xorshift128PlusX4 simd", |b| {
            b.iter(|| {
                fill_f64(&mut rng, &mut black_box(&mut v));
            })
        });
    }
    {
        use rand::SeedableRng;
        let mut uni = SmallRng::from_rng(thread_rng())
            .unwrap()
            .sample_iter(rand::distributions::Open01);
        c.bench_function("SmallRng", |b| {
            b.iter(|| {
                v.iter_mut().for_each(|x| *x = uni.next().unwrap());
            })
        });
    }
    let mut fancy = dSFMT::new();
    c.bench_function("dSFMT", |b| {
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
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
