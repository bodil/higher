use core::convert::identity;

/// A `Bifunctor` lets you change the types of a generic type with two type
/// parameters.
///
/// A `Bifunctor` works just like a [`Functor`](crate::Functor), but for types
/// with two type parameters. It will convert a `F<_, _>: Bifunctor` from `F<A,
/// B>` to `F<C, D>` using two functions, one `Fn(A) -> C` and the other `Fn(B)
/// -> D`.
pub trait Bifunctor<'a, A, B> {
    type Target<T, U>;

    /// Map a `Bifunctor<A, B>` to a `Bifunctor<C, D>` using a function from `A`
    /// to `C` and a function from `B` to `D`.
    fn bimap<C, D, L, R>(self, left: L, right: R) -> Self::Target<C, D>
    where
        L: Fn(A) -> C + 'a,
        R: Fn(B) -> D + 'a;

    /// Map only the left hand side of the bifunctor from `A` to `C`.
    fn lmap<C, L>(self, left: L) -> Self::Target<C, B>
    where
        Self: Sized,
        B: 'a,
        L: Fn(A) -> C + 'a,
    {
        self.bimap(left, identity)
    }

    /// Map only the right hand side of the bifunctor from `B` to `D`.
    fn rmap<D, R>(self, right: R) -> Self::Target<A, D>
    where
        Self: Sized,
        A: 'a,
        R: Fn(B) -> D + 'a,
    {
        self.bimap(identity, right)
    }
}

impl<A, B> Bifunctor<'_, A, B> for Result<A, B> {
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

impl<A, B> Bifunctor<'_, A, B> for Vec<(A, B)> {
    type Target<T, U> = Vec<(T, U)>;

    fn bimap<C, D, L, R>(self, left: L, right: R) -> Self::Target<C, D>
    where
        L: Fn(A) -> C,
        R: Fn(B) -> D,
    {
        self.into_iter().map(|(a, b)| (left(a), right(b))).collect()
    }
}
