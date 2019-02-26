use crate::Bifunctor;
use higher::Bilift;

/// A `Profunctor` is just a `Bifunctor` that is contravariant over its first
/// argument and covariant over its second argument.
///
/// What's the problem?
pub trait Profunctor<A, B, C, D>: Bilift<A, B, C, D> + Bifunctor<C, B, A, D> {
    fn dimap<L, R>(self, left: L, right: R) -> <Self as Bilift<A, B, C, D>>::Target
    where
        L: Fn(C) -> A,
        R: Fn(B) -> D;
}

pub trait ProfunctorLeft<A, B, C>: Profunctor<A, B, C, B> {
    fn lcmap<F>(self, f: F) -> <Self as Bilift<A, B, C, B>>::Target
    where
        F: Fn(C) -> A;
}

impl<A, B, C> ProfunctorLeft<A, B, C> for A
where
    A: Profunctor<A, B, C, B>,
{
    fn lcmap<F>(self, f: F) -> <Self as Bilift<A, B, C, B>>::Target
    where
        F: Fn(C) -> A,
    {
        self.dimap(f, |a| a)
    }
}
