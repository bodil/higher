use std::convert::identity;

/// A `Profunctor` is just a `Bifunctor` that is contravariant over its first
/// argument and covariant over its second argument. What's the problem?
pub trait Profunctor<'a, B: 'a, C: 'a> {
    type Target<T: 'a, U: 'a>;

    /// Map a function over both arguments of the profunctor.
    fn dimap<A: 'a, D: 'a, L: 'a, R: 'a>(self, left: L, right: R) -> Self::Target<A, D>
    where
        L: Fn(A) -> B,
        R: Fn(C) -> D;

    /// Map a function over the contravariant first argument only.
    fn lcmap<A: 'a, L: 'a>(self, left: L) -> Self::Target<A, C>
    where
        Self: Sized,
        L: Fn(A) -> B,
    {
        self.dimap(left, identity)
    }

    /// Map a function over the covariant second argument only.
    fn rmap<D: 'a, R: 'a>(self, right: R) -> Self::Target<B, D>
    where
        Self: Sized,
        R: Fn(C) -> D,
    {
        self.dimap(identity, right)
    }
}
