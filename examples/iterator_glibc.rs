extern crate wrand;
use wrand::{lgc, Gen};
fn main() {
    let mut rng = lgc::Lgcglibc::with_seed(-1i32 as u32);

    let iter = rng.geniter().take(100).map(|x| x % (100 - 1 + 1) + 1);

    let b: Box<_> = iter.collect();

    for i in b.iter() {
        println!("{}", i);
    }

    println!("==============================================");

    for _ in 0..20 {
        println!("{}", rng.gen_range(1..101));
    }
}
