use core::convert::identity;

/// A `Profunctor` is just a `Bifunctor` that is contravariant over its first
/// argument and covariant over its second argument. What's the problem?
pub trait Profunctor<'a, B, C> {
    type Target<T, U>;

    /// Map a function over both arguments of the profunctor.
    fn dimap<A, D, L, R>(self, left: L, right: R) -> Self::Target<A, D>
    where
        L: Fn(A) -> B + 'a,
        R: Fn(C) -> D + 'a;

    /// Map a function over the contravariant first argument only.
    fn lcmap<A, L>(self, left: L) -> Self::Target<A, C>
    where
        Self: Sized,
        C: 'a,
        L: Fn(A) -> B + 'a,
    {
        self.dimap(left, identity)
    }

    /// Map a function over the covariant second argument only.
    fn rmap<D, R>(self, right: R) -> Self::Target<B, D>
    where
        Self: Sized,
        B: 'a,
        R: Fn(C) -> D + 'a,
    {
        self.dimap(identity, right)
    }
}
