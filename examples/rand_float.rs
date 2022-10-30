extern crate wrand;
use wrand::{genf64, randomf64, rng, Gen, Random, RandomTrait};
fn main() {
    let mut rng = rng::Lgcglibctypen::new();
    for _ in 0..20 {
        println!("{:.8}", genf64((rng.gen(), rng.gen())));
    }

    println!("=============================");

    let mut rng = rng::Lgcmsvcrt::new();

    let mut rnd = Random::new_fromiter(std::iter::repeat_with(move || {
        randomf64((rng.gen(), rng.gen()), 1.0f64, 10.0f64).unwrap()
    }));

    for _ in 0..20 {
        println!("{:.8}", rnd.rand().unwrap());
    }
}
