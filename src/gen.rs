use std::ops::{Add, Rem, Sub};
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

pub fn random<T: Gen>(gen: &mut T, a: T::Output, b: T::Output) -> Option<T::Output>
where
    <T as Gen>::Output: Copy
        + PartialOrd
        + From<u32>
        + Add<Output = T::Output>
        + Sub<Output = T::Output>
        + Rem<Output = T::Output>,
{
    if a <= b {
        Some(a + gen.gen() % (b - a + 1u32.into()))
    } else {
        None
    }
}

pub fn randomf64<T: Gen>(gen: &mut T) -> f64
where
    <T as Gen>::Output: Copy
        + PartialOrd
        + From<u32>
        + Into<f64>
        + Add<Output = T::Output>
        + Sub<Output = T::Output>
        + Rem<Output = T::Output>,
{
    (random(gen, 0u32.into(), 10000u32.into()).unwrap().into() / (10000 as f64))
        * (random(gen, 0u32.into(), 10000u32.into()).unwrap().into() / (10000 as f64))
}
