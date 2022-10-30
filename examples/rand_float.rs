extern crate wrand;
use wrand::{genf64, randomf64, rng, Gen};
fn main() {
    let mut rng = rng::Lgcglibctypen::new();
    for _ in 0..20 {
        println!("{:.8}", genf64((rng.gen(), rng.gen())));
    }

    println!("=============================");

    let mut rng = rng::Lgcmsvcrt::new();

    let iter =
        std::iter::repeat_with(|| randomf64((rng.gen(), rng.gen()), 1.0f64, 10.0f64).unwrap())
            .take(20);

    for i in iter {
        println!("{:.8}", i);
    }
}
