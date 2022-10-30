extern crate wrand;
use wrand::{random, rng, Gen, Random, RandomTrait};
fn main() {
    let mut rng = rng::Lgcmsvcrt::new();

    let mut rnd = Random::new_fromiter(std::iter::repeat_with(|| {
        random(rng.gen(), 1, 100).unwrap()
    }));

    for _ in 0..100 {
        println!("{}", rnd.rand().unwrap());
    }

    println!("==============================");

    let mut rnd_iter = rnd.into_iter();

    for _ in 0..100 {
        println!("{}", rnd_iter.next().unwrap());
    }

    println!("==============================");

    let mut rnd = Random::new_fromiter(rnd_iter);

    for _ in 0..100 {
        println!("{}", rnd.rand().unwrap());
    }
}
