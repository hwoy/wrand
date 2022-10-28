extern crate wrand;
use wrand::{lgc, Gen};
fn main() {
    let iter = lgc::Lgcglibc::with_seed(-1i32 as u32)
        .into_iter()
        .take(100)
        .map(|x| x % (100 - 1 + 1) + 1);

    let b: Box<_> = iter.collect();

    for i in b.iter() {
        println!("{}", i);
    }
}
