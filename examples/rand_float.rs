extern crate wrand;
use wrand::{randomf64, rng, Gen};
fn main() {
    let mut rng = rng::Lgcglibctypen::new();
    for _ in 0..20 {
        println!("{:.8}", randomf64(rng.gen()));
    }

    println!("=============================");

    let mut rng = rng::Lgcmsvcrt::new();
    for _ in 0..20 {
        println!("{:.8}", randomf64(rng.gen()));
    }
}
