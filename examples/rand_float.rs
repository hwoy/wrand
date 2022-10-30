extern crate wrand;
use wrand::{from_iter, genf64, randomf64, rng, Gen, GenIterTrait};
fn main() {
    let mut rng = rng::Lgcglibctypen::new();
    for _ in 0..20 {
        println!("{:.8}", genf64((rng.gen(), rng.gen())));
    }

    println!("=============================");

    let mut rng = rng::Lgcmsvcrt::new();

    let mut rng = from_iter(std::iter::repeat_with(move || {
        randomf64((rng.gen(), rng.gen()), 1.0f64, 10.0f64).unwrap()
    }));

    for _ in 0..20 {
        println!("{:.8}", rng.gen().unwrap());
    }
}
