pub mod seed {
    use std::time::SystemTime;
    pub fn getseed() -> u32 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .subsec_micros() as u32
    }
}

use std::ops::Range;
pub trait Gen {
    fn gen(&mut self) -> i32;

    fn gen_range(&mut self, r: Range<i32>) -> i32 {
        r.start + (self.gen() % (r.end - r.start))
    }
}

pub fn random<T: Gen>(gen: &mut T, a: i32, b: i32) -> Option<i32> {
    if b < a {
        Some(gen.gen_range(a..(b + 1)))
    } else {
        None
    }
}

pub mod lgc {
    use super::seed;
    use super::Gen;
    use std::vec::Vec;

    #[derive(Clone, Copy)]
    pub struct Lgc {
        state: u32,
        a: u32,
        c: u32,
        m: u64,
    }

    impl Lgc {
        pub fn new(seed: u32, a: u32, c: u32, m: u64) -> Self {
            Self {
                state: seed,
                a: a,
                c: c,
                m: m,
            }
        }
    }

    impl Gen for Lgc {
        fn gen(&mut self) -> i32 {
            self.state = (self.state.wrapping_mul(self.a).wrapping_add(self.c) as u64)
                .wrapping_rem(self.m) as u32;
            self.state as i32
        }
    }

    #[derive(Clone, Copy)]
    pub struct Lgcglibc {
        lgc: Lgc,
    }

    impl Lgcglibc {
        pub fn with_seed(seed: u32) -> Self {
            Self {
                lgc: Lgc::new(
                    seed & ((1u32 << 31) - 1u32),
                    1103515245u32,
                    12345u32,
                    1u64 << 31,
                ),
            }
        }
        pub fn new() -> Self {
            Self::with_seed(seed::getseed())
        }
    }

    impl Gen for Lgcglibc {
        fn gen(&mut self) -> i32 {
            self.lgc.gen();
            self.lgc.state &= (1u32 << 31) - 1u32;
            self.lgc.state as i32
        }
    }

    #[derive(Clone, Copy)]
    pub struct Lgcmsvcrt {
        lgc: Lgc,
    }

    impl Lgcmsvcrt {
        pub fn with_seed(seed: u32) -> Self {
            Self {
                lgc: Lgc::new(
                    seed & ((1u32 << 31) - 1u32),
                    214013u32,
                    2531011u32,
                    1u64 << 32,
                ),
            }
        }
        pub fn new() -> Self {
            Self::with_seed(seed::getseed())
        }
    }

    impl Gen for Lgcmsvcrt {
        fn gen(&mut self) -> i32 {
            self.lgc.gen();
            self.lgc.state &= (1u32 << 31) - 1u32;
            (self.lgc.state >> 16) as i32
        }
    }

    #[derive(Clone)]
    pub struct Lgcglibctypen {
        states: Vec<i32>,
    }

    impl Lgcglibctypen {
        pub fn with_seed(seed: u32) -> Self {
            let mut buffer = Vec::<i32>::with_capacity(344usize);
            buffer.resize(344usize, 0);
            let buffer = buffer.as_mut_slice();

            buffer[0] = seed as i32;

            for i in 1..31usize {
                buffer[i] = ((16807i64.wrapping_mul(buffer[i - 1] as i64))
                    .wrapping_rem(2147483647i64)) as i32;

                if buffer[i] < 0 {
                    buffer[i] = buffer[i].wrapping_add(2147483647);
                }
            }

            for i in 31..34usize {
                buffer[i] = buffer[i - 31];
            }

            for i in 34..344usize {
                buffer[i] = buffer[i - 31].wrapping_add(buffer[i - 3]);
            }

            let states: Vec<i32> = buffer.iter().rev().take(31usize).rev().copied().collect();

            Self { states: states }
        }
        pub fn new() -> Self {
            Self::with_seed(seed::getseed())
        }
    }

    impl Gen for Lgcglibctypen {
        fn gen(&mut self) -> i32 {
            let states = self.states.as_mut_slice();
            let len = states.len();

            let val = states[len - 31].wrapping_add(states[len - 3]);
            states.rotate_left(1);
            states[len - 1] = val;
            ((val as u32) >> 1) as i32
        }
    }
}
