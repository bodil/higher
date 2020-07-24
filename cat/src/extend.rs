use crate::Functor;
use higher::Lift;

/// `Extend` is the opposite of `Bind`.
pub trait Extend<A, B>: Functor<A, B> + Sized {
    fn extend<F>(self, f: F) -> <Self as Lift<A, B>>::Target1
    where
        F: Fn(Self) -> B;
}

#[cfg(feature = "std")]
impl<A, B> Extend<A, B> for Vec<A>
where
    A: Clone,
{
    fn extend<F>(self, f: F) -> <Self as Lift<A, B>>::Target1
    where
        F: Fn(Self) -> B,
    {
        (0..self.len())
            .map(|index| f(self.iter().skip(index).cloned().collect()))
            .collect()
    }
}
