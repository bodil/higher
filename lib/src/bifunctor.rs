use crate::Bilift;

/// A `Functor` over two arguments.
pub trait Bifunctor<A, B, C, D>: Bilift<A, B, C, D> {
    fn bimap<L, R>(self, left: L, right: R) -> <Self as Bilift<A, B, C, D>>::Target
    where
        L: Fn(A) -> C,
        R: Fn(B) -> D;
}

pub trait BifunctorLeft<A, B, C>: Bifunctor<A, B, C, B> {
    fn lmap<F>(self, f: F) -> <Self as Bilift<A, B, C, B>>::Target
    where
        F: Fn(A) -> C;
}

impl<A, B, C> BifunctorLeft<A, B, C> for A
where
    A: Bifunctor<A, B, C, B>,
{
    fn lmap<F>(self, f: F) -> <Self as Bilift<A, B, C, B>>::Target
    where
        F: Fn(A) -> C,
    {
        self.bimap(f, |a| a)
    }
}

pub trait BifunctorRight<A, B, C>: Bifunctor<A, B, A, C> {
    fn rmap<F>(self, f: F) -> <Self as Bilift<A, B, A, C>>::Target
    where
        F: Fn(B) -> C;
}

impl<A, B, C> BifunctorRight<A, B, C> for A
where
    A: Bifunctor<A, B, A, C>,
{
    fn rmap<F>(self, f: F) -> <Self as Bilift<A, B, A, C>>::Target
    where
        F: Fn(B) -> C,
    {
        self.bimap(|a| a, f)
    }
}

impl<A, B, C, D> Bifunctor<A, B, C, D> for Result<A, B> {
    fn bimap<L, R>(self, left: L, right: R) -> <Self as Bilift<A, B, C, D>>::Target
    where
        L: Fn(A) -> C,
        R: Fn(B) -> D,
    {
        match self {
            Ok(a) => Ok(left(a)),
            Err(b) => Err(right(b)),
        }
    }
}
