extern crate wrand;
use wrand::{randomf64, rng};
fn main() {
    let mut rng = rng::Lgcglibctypen::new();

    for _ in 0..20 {
        println!("{:.4}", randomf64(&mut rng));
    }
}
