extern crate wrand;
use wrand::{from_iter, random, rng, Gen, GenIterTrait};
fn main() {
    let mut rng = rng::Lgcmsvcrt::new();

    let mut rng_iter = from_iter(std::iter::repeat_with(|| {
        random(rng.gen(), 1, 100).unwrap()
    }));

    for _ in 0..100 {
        println!("{}", rng_iter.gen().unwrap());
    }

    println!("==============================");

    let mut rng_iter = from_iter(std::iter::repeat_with(|| {
        random(rng.gen(), 10, 20).unwrap()
    }));

    for _ in 0..100 {
        println!("{}", rng_iter.gen().unwrap());
    }
}
