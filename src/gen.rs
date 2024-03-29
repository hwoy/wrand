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

pub trait RandomTrait {
    type Output;
    fn rand(&mut self) -> Self::Output;
}

pub struct Random<I> {
    iter: I,
}

impl<A, I> Random<I>
where
    I: Iterator<Item = A>,
{
    #[inline]
    pub fn new_fromiter(iter: I) -> Random<I> {
        Random { iter: iter }
    }

    #[inline]
    pub fn into_iter(self) -> I {
        self.iter
    }
}

impl<A, I> RandomTrait for Random<I>
where
    I: Iterator<Item = A>,
{
    type Output = Option<I::Item>;
    #[inline]
    fn rand(&mut self) -> Self::Output {
        self.iter.next()
    }
}
