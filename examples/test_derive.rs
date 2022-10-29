extern crate wrand;
use wrand::{rng, Gen};

fn showgen<T: Gen<Output = wrand::RANDTYPE>>(rng: &mut T, n: usize) {
    for _ in 0..n {
        println!("{}", wrand::random(rng, 0, 100).unwrap());
    }
}

fn main() {
    let mut rng = rng::Lgcglibctypen::with_seed(-1i32 as u32);

    println!("======== Lgcglibctypen ==========");
    showgen(&mut rng, 10usize);

    let mut rng = rng::Lgcglibc::with_seed(-1i32 as u32);

    println!("======== Lgcglibc ==========");
    showgen(&mut rng, 10usize);

    let mut rng = rng::Lgcmsvcrt::with_seed(-1i32 as u32);

    println!("======== Lgcmsvcrt ==========");
    showgen(&mut rng, 10usize);

    println!("******************* after reset *******************");

    let mut rng = rng::Lgcglibctypen::with_seed(-1i32 as u32);

    println!("======== Lgcglibctypen ==========");
    showgen(&mut rng, 10usize);

    let mut rng = rng::Lgcglibc::with_seed(-1i32 as u32);

    println!("======== Lgcglibc ==========");
    showgen(&mut rng, 10usize);

    let mut rng = rng::Lgcmsvcrt::with_seed(-1i32 as u32);

    println!("======== Lgcmsvcrt ==========");
    showgen(&mut rng, 10usize);
}
