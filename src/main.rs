use agent_based_trading_julia::{cont_run, FRNG, dSFMT};

fn main() {
    for _ in 0..100 {
        cont_run(
        10_000,
        10_000,
        0.05,
        0.1
        );
    }
    // let mut rng = FRNG::new();
    // let mut rng = dSFMT::new();

    // let mut v = vec![0.; 1000];
    // rng.fill(&mut v);
    // for x in v {
    //     println!("{}", x);
    // }
}
