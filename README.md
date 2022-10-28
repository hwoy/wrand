# wrand

wrand is a very simple random library for Rust.

## Features

1. Full open source.
2. Provides iterator.
3. Provides basic lgc PRNG, msvcrt PRNG, glibc PRNG

```Rust

use wrand::{lgc, Gen};
fn main() {
    let mut lgc = lgc::Lgcglibctypen::with_seed(-1i32 as u32);

    for _ in 0..10 {
        println!("{}", lgc.gen());
    }
}

```

## Contact me

- Web: <https://github.com/hwoy>
- Email: <mailto:bosskillerz@gmail.com>
- Facebook: <http://www.facebook.com/dead-root>
