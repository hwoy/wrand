extern crate wrand;
use wrand::{rng, Gen};
fn main() {
    let mut rng = rng::Lgcglibctypen::with_seed(-1i32 as u32);

    for _ in 0..10 {
        println!("{}", rng.gen());
    }
}
