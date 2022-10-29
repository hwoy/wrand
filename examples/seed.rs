extern crate wrand;
fn main() {
    for _ in 0..20 {
        println!("{}", wrand::seed::getseed());
    }
}
