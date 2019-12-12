extern crate wrand;
use wrand::{lgc, Gen};
fn main() {
    let mut lgc = lgc::Lgcglibc::with_seed(-1i32 as u32);

    for _ in 0..10 {
        println!("{}", lgc.gen());
    }
}
