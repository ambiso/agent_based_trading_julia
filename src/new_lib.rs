// resides in src/lib.rs
use rand::prelude::SmallRng;
use rand::{thread_rng, Rng, SeedableRng};

pub fn cont_run(time: usize, n: usize, lambda: f64, q: f64) -> f64 {
    let mut theta = vec![0.; n];
    let n = n as f64;

    let mut eps_sampler = SmallRng::from_rng(thread_rng())
        .unwrap()
        .sample_iter(rand_distr::StandardNormal);
    let mut pchange_sampler = SmallRng::from_rng(thread_rng())
        .unwrap()
        .sample_iter(rand::distributions::Uniform::new_inclusive(0., 1.));

    let mut r = Vec::with_capacity(time);
    for t in 0..r.len() {
        let eps: f64 = eps_sampler.next().unwrap();
        let r_t = if eps > 0. {
            theta.iter().filter(|&&x| eps > x).count() as f64 / (lambda * n)
        } else {
            -(theta.iter().filter(|&&x| -eps > x).count() as f64) / (lambda * n)
        };
        theta
            .iter_mut()
            .filter(|_| pchange_sampler.next().unwrap() < q)
            .for_each(|x| {
                *x = r_t.abs();
            });
        r.push(r_t);
    }
    kurtosis(r)
}

fn kurtosis(x: Vec<f64>) -> f64 {
    let n = x.len() as f64;
    let mean_x = x.iter().sum::<f64>() / n;
    let x = x.iter().copied().map(|x| x - mean_x);
    let r: f64 = n * x.clone().map(|x| x.clone().powi(4)).sum::<f64>()
        / (x.map(|x| x.powi(2)).sum::<f64>().powi(2));

    r * (1. - 1. / n).powi(2) - 3.
}
