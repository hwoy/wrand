extern crate wrand;
use wrand::{rng, Gen};
fn main() {
    let mut rng = rng::Lgcglibc::with_seed(-1i32 as u32);

    let iter = std::iter::repeat_with(|| rng.gen())
        .take(100)
        .map(|x| x % (100 - 1 + 1) + 1);

    let b: Box<_> = iter.collect();

    for i in b.iter() {
        println!("{}", i);
    }

    println!("==============================================");

    for _ in 0..20 {
        println!("{}", wrand::random(rng.gen(), 1, 100).unwrap());
    }
}
