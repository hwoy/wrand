use std::ops::{Add, Mul, Rem, Sub};
pub trait Gen {
    type Output;

    fn gen(&mut self) -> Self::Output;
}

#[inline]
pub fn random<T>(value: T, a: T, b: T) -> Option<T>
where
    T: Copy + PartialOrd + From<u32> + Add<Output = T> + Sub<Output = T> + Rem<Output = T>,
{
    if a <= b {
        Some(a + value % (b - a + 1u32.into()))
    } else {
        None
    }
}

pub fn genf64<T>((value1, value2): (T, T)) -> f64
where
    T: Copy
        + PartialOrd
        + From<u32>
        + Into<f64>
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Rem<Output = T>,
{
    (random(value1, 0u32.into(), 10000u32.into()).unwrap()
        * random(value2, 0u32.into(), 10000u32.into()).unwrap())
    .into()
        / (100000000u32 as f64)
}

#[inline]
pub fn randomf64<T>((value1, value2): (T, T), a: f64, b: f64) -> Option<f64>
where
    T: Copy
        + PartialOrd
        + From<u32>
        + Into<f64>
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Rem<Output = T>,
{
    if a <= b {
        Some(a + genf64((value1, value2)) * (b - a))
    } else {
        None
    }
}

pub trait GenIterTrait {
    type Output;
    fn gen(&mut self) -> Option<Self::Output>;
}

pub struct GenIter<I> {
    iter: I,
}

impl<A, I> GenIterTrait for GenIter<I>
where
    I: Iterator<Item = A>,
{
    type Output = I::Item;
    #[inline]
    fn gen(&mut self) -> Option<Self::Output> {
        self.iter.next()
    }
}

#[inline]
pub fn from_iter<A, I: Iterator<Item = A>>(iter: I) -> GenIter<I> {
    GenIter { iter: iter }
}
