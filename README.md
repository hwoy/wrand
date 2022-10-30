# wrand

wrand stands for Hwoy's random, is a very simple random library for Rust.

## Features

1. Full open source.
2. Provides iterator(Removed).
3. Provides basic lgc PRNG, msvcrt PRNG, glibc PRNG

```Rust

extern crate wrand;
use wrand::{from_iter, random, rng, Gen, RandomTrait};
fn main() {
    let mut rng = rng::Lgcmsvcrt::new();

    let mut rnd = from_iter(std::iter::repeat_with(|| {
        random(rng.gen(), 1, 100).unwrap()
    }));

    for _ in 0..100 {
        println!("{}", rnd.rand().unwrap());
    }

    println!("==============================");

    let mut rnd = from_iter(std::iter::repeat_with(|| {
        random(rng.gen(), 10, 20).unwrap()
    }));

    for _ in 0..100 {
        println!("{}", rnd.rand().unwrap());
    }
}

```

## Contact me

- Web: <https://github.com/hwoy>
- Email: <mailto:bosskillerz@gmail.com>
- Facebook: <http://www.facebook.com/dead-root>
