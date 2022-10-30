use std::ops::{Add, Mul, Rem, Sub};
pub trait Gen {
    type Output;

    fn gen(&mut self) -> Self::Output;

    fn geniter(&mut self) -> GenIterator<Self>
    where
        Self: Sized,
    {
        GenIterator { gen: self }
    }

    fn into_geniter(self) -> IntoGenIterator<Self>
    where
        Self: Sized,
    {
        IntoGenIterator { gen: self }
    }
}

pub struct GenIterator<'a, T: Gen + 'a> {
    gen: &'a mut T,
}

impl<T: Gen> Iterator for GenIterator<'_, T> {
    type Item = T::Output;
    fn next(&mut self) -> Option<T::Output> {
        Some(self.gen.gen())
    }
}

pub struct IntoGenIterator<T: Gen> {
    gen: T,
}

impl<T: Gen> Iterator for IntoGenIterator<T> {
    type Item = T::Output;
    fn next(&mut self) -> Option<T::Output> {
        Some(self.gen.gen())
    }
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

pub struct Genf64 {
    val: f64,
}

pub fn genf64<T: Gen>(gen: &mut T) -> Genf64
where
    <T as Gen>::Output: Copy
        + PartialOrd
        + From<u32>
        + Into<f64>
        + Add<Output = T::Output>
        + Sub<Output = T::Output>
        + Mul<Output = T::Output>
        + Rem<Output = T::Output>,
{
    Genf64 {
        val: (random(gen.gen(), 0u32.into(), 10000u32.into()).unwrap()
            * random(gen.gen(), 0u32.into(), 10000u32.into()).unwrap())
        .into()
            / (100000000u32 as f64),
    }
}

#[inline]
pub fn randomf64(Genf64 { val }: Genf64, a: f64, b: f64) -> Option<f64> {
    if a <= b {
        Some(a + val * (b - a))
    } else {
        None
    }
}
