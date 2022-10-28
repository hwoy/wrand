extern crate wrand;
use wrand::{lgc, Gen};

fn showgen<T: Gen<Output = wrand::RANDTYPE>>(gen: &mut T, n: usize) {
    for _ in 0..n {
        println!("{}", gen.gen_range(0..100));
    }
}

fn main() {
    let mut genglibn = lgc::Lgcglibctypen::with_seed(-1i32 as u32);

    println!("======== Lgcglibctypen ==========");
    showgen(&mut genglibn, 10usize);

    let mut genglib = lgc::Lgcglibc::with_seed(-1i32 as u32);

    println!("======== Lgcglibc ==========");
    showgen(&mut genglib, 10usize);

    let mut genms = lgc::Lgcmsvcrt::with_seed(-1i32 as u32);

    println!("======== Lgcmsvcrt ==========");
    showgen(&mut genms, 10usize);

    println!("******************* after reset *******************");

    genglibn = lgc::Lgcglibctypen::with_seed(-1i32 as u32);

    println!("======== Lgcglibctypen ==========");
    showgen(&mut genglibn, 10usize);

    genglib = lgc::Lgcglibc::with_seed(-1i32 as u32);

    println!("======== Lgcglibc ==========");
    showgen(&mut genglib, 10usize);

    genms = lgc::Lgcmsvcrt::with_seed(-1i32 as u32);

    println!("======== Lgcmsvcrt ==========");
    showgen(&mut genms, 10usize);
}
