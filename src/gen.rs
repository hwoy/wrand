pub type SEEDTYPE = u32;
pub type RANDTYPE = u32;

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
