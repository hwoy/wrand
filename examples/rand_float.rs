extern crate wrand;
use wrand::{randomf64, rng, Gen};
fn main() {
    let mut rng = rng::Lgcglibctypen::new();
    for _ in 0..20 {
        println!("{:.8}", randomf64(rng.gen()));
    }

    println!("=============================");

    let iter = rng::Lgcmsvcrt::new()
        .into_geniter()
        .take(20)
        .map(|x| randomf64(x));
    for i in iter {
        println!("{:.8}", i);
    }
}
