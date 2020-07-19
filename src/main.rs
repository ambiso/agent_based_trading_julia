use agent_based_trading_julia::fill_f64;
use rand::SeedableRng;
use simd_prngs::Xorshift128PlusX4;

fn main() {
    let mut v = vec![0.; 10_000];
    let mut rng = Xorshift128PlusX4::from_rng(rand::thread_rng()).unwrap();
    let mut k = 0.;
    for _ in 0..1_000_000 {
        fill_f64(&mut rng, &mut v);
        k += v[9999];
    }
    println!("{}", k);
    // use agent_based_trading_julia::cont_run;
}
