use crate::Functor;

/// `Extend` is the opposite of `Bind`.
pub trait Extend<A>: Functor<A> {
    fn extend<F, B>(self, f: F) -> Self::Target<B>
    where
        Self: Sized,
        F: Fn(Self) -> B;
}

#[cfg(feature = "std")]
<<<<<<< HEAD:prelude/src/extend.rs
impl<A> Extend<A> for Vec<A>
=======
impl<A, B> Extend<A, B> for Vec<A>
>>>>>>> master:cat/src/extend.rs
where
    A: Clone,
{
    fn extend<F, B>(self, f: F) -> Self::Target<B>
    where
        Self: Sized,
        F: Fn(Self) -> B,
    {
        (0..self.len())
            .map(|index| f(self.iter().skip(index).cloned().collect()))
            .collect()
    }
}
