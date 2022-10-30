extern crate wrand;
use wrand::{from_iter, random, rng, Gen, RandomTrait};
fn main() {
    let mut rng = rng::Lgcmsvcrt::new();

    let mut rnd = from_iter(std::iter::repeat_with(|| {
        random(rng.gen(), 1, 100).unwrap()
    }));

    for _ in 0..100 {
        println!("{}", rnd.rand().unwrap());
    }

    println!("==============================");

    let mut rnd = from_iter(std::iter::repeat_with(|| {
        random(rng.gen(), 10, 20).unwrap()
    }));

    for _ in 0..100 {
        println!("{}", rnd.rand().unwrap());
    }
}
