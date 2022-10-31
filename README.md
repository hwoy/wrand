# wrand

- wrand stands for Hwoy's random, is a very simple random library for Rust.
- Yet another rand crate.

## Features

1. Full open source.
2. Provides iterator.
3. Provides basic lgc PRNG, msvcrt PRNG, glibc PRNG

```Rust

extern crate wrand;
use wrand::{random, rng, Gen, Random, RandomTrait};
fn main() {
    let mut rng = rng::Lgcmsvcrt::new();

    let mut rnd = Random::new_fromiter(std::iter::repeat_with(|| {
        random(rng.gen(), 1, 100).unwrap()
    }));

    for _ in 0..100 {
        println!("{}", rnd.rand().unwrap());
    }

    println!("==============================");

    let mut rnd_iter = rnd.into_iter();

    for _ in 0..100 {
        println!("{}", rnd_iter.next().unwrap());
    }

    println!("==============================");

    let mut rnd = Random::new_fromiter(rnd_iter);

    for _ in 0..100 {
        println!("{}", rnd.rand().unwrap());
    }
}


```

## Contact me

- Web: <https://github.com/hwoy>
- Email: <mailto:bosskillerz@gmail.com>
- Facebook: <http://www.facebook.com/dead-root>
