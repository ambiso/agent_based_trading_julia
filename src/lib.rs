// resides in src/lib.rs
use rand::prelude::SmallRng;
use rand::{thread_rng, Rng, RngCore, SeedableRng};
use std::time::{Duration, Instant};

pub struct FRNG {
    inner: SmallRng,
}

fn rand_to_f64(x: u64) -> f64 {
    unsafe { std::mem::transmute::<_, f64>((x & ((1 << 52) - 1)) | (0x3fe << 52)) }
}

impl FRNG {
    pub fn new() -> Self {
        Self {
            inner: SmallRng::from_entropy(),
        }
    }

    pub fn gen(&mut self) -> f64 {
        rand_to_f64(self.inner.next_u64())
    }

    pub fn fill(&mut self, v: &mut [f64]) {
        unsafe {
            use std::slice;
            self.inner.fill_bytes(slice::from_raw_parts_mut(v.as_mut_ptr() as *mut u8, v.len() * 8));
            v.iter_mut().for_each(|x| *x = rand_to_f64(std::mem::transmute(*x)));
        }
    }
}

#[repr(C)]
pub struct dSFMT {
    _private: [u8; 4096]
}

#[link(name = "dSFMT", kind = "static")]
extern {
    fn dsfmt_init_gen_rand(gen: *mut dSFMT, seed: u32);
    fn dsfmt_fill_array_open_close(gen: *mut dSFMT, array: *mut f64, size: libc::c_int);
}

impl dSFMT {
    pub fn new() -> Self {
        let mut s = Self { _private: [0; 4096] };
        unsafe {
            dsfmt_init_gen_rand(&mut s, 1337);
        }
        s
    }

    pub fn fill(&mut self, v: &mut [f64]) {
        unsafe {
            dsfmt_fill_array_open_close(self, v.as_mut_ptr(), v.len() as libc::c_int);
        }
    }
}

pub fn cont_run(time: usize, n: usize, lambda: f64, q: f64) -> f64 {
    let mut eps_sampler = SmallRng::from_rng(thread_rng())
        .unwrap()
        .sample_iter(rand_distr::StandardNormal);

    let mut pchange = vec![0.0; n];
    let mut r = vec![0.0; time];
    let mut theta = vec![0.; n];
    let mut rng = dSFMT::new();

    let n = n as f64;
    for t in 0..time {
        let eps: f64 = eps_sampler.next().unwrap();
        let r_t = if eps > 0. {
                theta.iter().map(|&x| (eps > x) as i32).sum::<i32>() as f64 / (lambda * n)
            } else {
                -(theta.iter().map(|&x| (-eps > x) as i32).sum::<i32>() as f64) / (lambda * n)
            };
        // pchange.iter_mut()
        //     .for_each(|x| *x = pchange_sampler.next().unwrap())
        rng.fill(&mut pchange);
        theta
            .iter_mut()
            .zip(pchange.iter())
            .for_each(|(x, pc)| {
                *x = if *pc < q { r_t.abs() } else { *x };
            });
        r[t] = r_t;
    }
    kurtosis(r)
}

fn kurtosis(x: Vec<f64>) -> f64 {
    let n = x.len() as f64;
    let mean_x = x.iter().sum::<f64>() / n;
    let x = x.iter().map(|x| *x - mean_x);
    let r: f64 = n * x.clone().map(|x| x.clone().powi(4)).sum::<f64>()
        / (x.map(|x| x.powi(2)).sum::<f64>().powi(2));

    r * (1. - 1. / n).powi(2) - 3.
}
