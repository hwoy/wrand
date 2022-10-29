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

pub fn randomf64<T>(gen: T) -> f64
where
    T: Copy
        + PartialOrd
        + From<u32>
        + Into<f64>
        + Add<Output = T>
        + Sub<Output = T>
        + Rem<Output = T>,
{
    (random(gen, 0u32.into(), 10000u32.into()).unwrap().into()
        * random(gen, 0u32.into(), 10000u32.into()).unwrap().into())
        / (100000000u32 as f64)
}
