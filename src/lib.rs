pub mod seed {
    use std::time::SystemTime;
    pub fn getseed() -> u32 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .subsec_micros() as u32
    }
}

use std::ops::{Add, Range, Rem, Sub};
pub trait Gen {
    type Output;

    fn gen(&mut self) -> Self::Output;

    fn gen_range(&mut self, r: Range<Self::Output>) -> Self::Output
    where
        <Self as Gen>::Output: Copy
            + Sub<Output = Self::Output>
            + Rem<Output = Self::Output>
            + Add<Output = Self::Output>,
    {
        r.start + (self.gen() % (r.end - r.start))
    }
}

pub fn random<T: Gen>(gen: &mut T, a: T::Output, b: T::Output) -> Option<T::Output>
where
    <T as Gen>::Output: Copy
        + PartialOrd
        + From<u8>
        + Add<Output = T::Output>
        + Sub<Output = T::Output>
        + Rem<Output = T::Output>,
{
    if a <= b {
        Some(gen.gen_range(a..(b + 1u8.into())))
    } else {
        None
    }
}

pub mod lgc {
    use super::seed;
    use super::Gen;

    #[derive(Clone, Copy)]
    pub struct Lgc {
        //type Output = i32;
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
        type Output = i32;
        fn gen(&mut self) -> Self::Output {
            self.state = (self.state.wrapping_mul(self.a).wrapping_add(self.c) as u64)
                .wrapping_rem(self.m) as u32;
            self.state as Self::Output
        }
    }

    #[derive(Clone, Copy)]
    pub struct Lgcglibc {
        //type Output = i32,
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
        type Output = i32;
        fn gen(&mut self) -> Self::Output {
            self.lgc.gen();
            self.lgc.state &= (1u32 << 31) - 1u32;
            self.lgc.state as Self::Output
        }
    }

    #[derive(Clone, Copy)]
    pub struct Lgcmsvcrt {
        //type Output = i32,
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
        type Output = i32;
        fn gen(&mut self) -> Self::Output {
            self.lgc.gen();
            self.lgc.state &= (1u32 << 31) - 1u32;
            (self.lgc.state >> 16) as Self::Output
        }
    }

    #[derive(Clone)]
    pub struct Lgcglibctypen {
        //type Output = i32;
        states: Box<[i32]>,
    }

    impl Lgcglibctypen {
        pub fn with_seed(seed: u32) -> Self {
            let mut buffer: Box<[i32]> = std::iter::repeat(0).take(344).collect();

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

            let states: Box<[i32]> = buffer.iter().rev().take(31).rev().copied().collect();

            Self { states: states }
        }
        pub fn new() -> Self {
            Self::with_seed(seed::getseed())
        }
    }

    impl Gen for Lgcglibctypen {
        type Output = i32;
        fn gen(&mut self) -> Self::Output {
            let states = &mut self.states;
            let len = states.len();

            let val = states[len - 31].wrapping_add(states[len - 3]);
            states.rotate_left(1);
            states[len - 1] = val;
            ((val as u32) >> 1) as Self::Output
        }
    }
}
