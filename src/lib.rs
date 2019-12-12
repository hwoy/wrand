/*

#include <stdio.h>

#define MAX 1000
#define seed 1

main() {
  int buffer[MAX];
  int i;

  buffer[0] = seed;
  for (i=1; i<31; i++) {
    buffer[i] = (16807LL * buffer[i-1]) % 2147483647;
    if (buffer[i] < 0) {
      buffer[i] += 2147483647;
    }
  }
  for (i=31; i<34; i++) {
    buffer[i] = buffer[i-31];
  }
  for (i=34; i<344; i++) {
    buffer[i] = buffer[i-31] + buffer[i-3];
  }

  for (i=344; i<MAX; i++) {
    buffer[i] = buffer[i-31] + buffer[i-3];
    printf("%d\n", ((unsigned int)buffer[i]) >> 1);
  }

  return 0;
}

*/

/*

#include <stdio.h>
#include <stdlib.h>

int main(void) {
  unsigned int i;
  srand(-1);

  for(i=0;i<10;++i)
    printf("%d\n",rand());
  return 0;
}

*/

#![crate_type = "lib"]

pub mod seed {
    use std::time::SystemTime;
    pub fn getseed() -> u32 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .subsec_micros() as u32
    }
}

pub trait Gen {
    fn gen(&mut self) -> i32;
}

pub fn random<T: Gen>(gen: &mut T, a: i32, b: i32) -> Option<i32> {
    if a > b {
        return None;
    }
    let moder = b - a + 1;
    Some(a + gen.gen() % moder)
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
            buffer.resize(344usize,0);
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

            let mut states= Vec::<i32>::with_capacity(31usize);
            states.resize(31usize,0);

            {
                let states = states.as_mut_slice();

                let start_offset = buffer.len()-states.len();

                let (_,buffer) = buffer.split_at(start_offset);

                states.copy_from_slice(buffer);
            }

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

            let val = states[len - 31]
                .wrapping_add(states[len - 3]);
            states.rotate_left(1);
            states[len - 1] = val;
            ((val as u32) >> 1) as i32
        }
    }
}
