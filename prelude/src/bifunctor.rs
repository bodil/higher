use std::convert::identity;

/// A `Bifunctor` lets you change the types of a generic type with two type
/// parameters.
///
/// A `Bifunctor` works just like a `Functor`, but for types with two type
/// parameters. It will convert a `F<_, _>: Bifunctor` from `F<A, B>` to
/// `F<C, D>` using two functions, one `Fn(A) -> C` and the other `Fn(B) -> D`.
pub trait Bifunctor<A, B> {
    type Target<T, U>;

    fn bimap<C, D, L, R>(self, left: L, right: R) -> Self::Target<C, D>
    where
        L: Fn(A) -> C,
        R: Fn(B) -> D;

    fn lmap<C, L>(self, left: L) -> Self::Target<C, B>
    where
        Self: Sized,
        L: Fn(A) -> C,
    {
        self.bimap(left, identity)
    }

    fn rmap<D, R>(self, right: R) -> Self::Target<A, D>
    where
        Self: Sized,
        R: Fn(B) -> D,
    {
        self.bimap(identity, right)
    }
}

impl<A, B> Bifunctor<A, B> for Result<A, B> {
    type Target<T, U> = Result<T, U>;

    fn bimap<C, D, L, R>(self, left: L, right: R) -> Self::Target<C, D>
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
