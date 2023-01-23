use std::convert::identity;

use crate::Functor;

/// A `Bifunctor` lets you change the types of a generic type with two type
/// parameters.
///
/// A `Bifunctor` works just like a [`Functor`](crate::Functor), but for types
/// with two type parameters. It will convert a `F<_, _>: Bifunctor` from `F<A,
/// B>` to `F<C, D>` using two functions, one `Fn(A) -> C` and the other `Fn(B)
/// -> D`.
pub trait Bifunctor<'a, A: 'a, B: 'a> {
    type Target<T: 'a, U: 'a>;

    /// Map a `Bifunctor<A, B>` to a `Bifunctor<C, D>` using a function from `A`
    /// to `C` and a function from `B` to `D`.
    fn bimap<C: 'a, D: 'a, L: 'a, R: 'a>(self, left: L, right: R) -> Self::Target<C, D>
    where
        L: Fn(A) -> C,
        R: Fn(B) -> D;

    /// Map only the left hand side of the bifunctor from `A` to `C`.
    fn lmap<C: 'a, L: 'a>(self, left: L) -> Self::Target<C, B>
    where
        Self: Sized,
        L: Fn(A) -> C,
    {
        self.bimap(left, identity)
    }

    /// Map only the right hand side of the bifunctor from `B` to `D`.
    fn rmap<D: 'a, R: 'a>(self, right: R) -> Self::Target<A, D>
    where
        Self: Sized,
        R: Fn(B) -> D,
    {
        self.bimap(identity, right)
    }
}

pub trait BifunctorRef<'a, A: 'a, B: 'a>: Bifunctor<'a, A, B> {
    /// Map a `Bifunctor<A, B>` to a `Bifunctor<C, D>` using a function from `&A`
    /// to `C` and a function from `&B` to `D`.
    fn bimap_ref<C: 'a, D: 'a, L: 'a, R: 'a>(&self, left: L, right: R) -> Self::Target<C, D>
    where
        L: Fn(&A) -> C,
        R: Fn(&B) -> D;
}

impl<'a, A: 'a, B: 'a> Bifunctor<'a, A, B> for Result<A, B> {
    type Target<T, U> = Result<T, U> where T: 'a, U: 'a;

    fn bimap<C, D, L, R>(self, left: L, right: R) -> Self::Target<C, D>
    where
        C: 'a,
        D: 'a,
        L: Fn(A) -> C,
        R: Fn(B) -> D,
    {
        match self {
            Ok(a) => Ok(left(a)),
            Err(b) => Err(right(b)),
        }
    }
}

impl<'a, A: 'a, B: 'a> BifunctorRef<'a, A, B> for Result<A, B> {
    fn bimap_ref<C: 'a, D: 'a, L: 'a, R: 'a>(&self, left: L, right: R) -> Self::Target<C, D>
    where
        L: Fn(&A) -> C,
        R: Fn(&B) -> D,
    {
        match self {
            Ok(a) => Ok(left(a)),
            Err(b) => Err(right(b)),
        }
    }
}

impl<'a, A: 'a, B: 'a, const N: usize> Bifunctor<'a, A, B> for [(A, B); N] {
    type Target<T, U> = [(T, U); N]
    where
        T: 'a,
        U: 'a;

    fn bimap<C, D, L, R>(self, left: L, right: R) -> Self::Target<C, D>
    where
        C: 'a,
        D: 'a,
        L: Fn(A) -> C + 'a,
        R: Fn(B) -> D + 'a,
    {
        self.fmap(|(a, b)| (left(a), right(b)))
    }
}

impl<'a, A: 'a, B: 'a> Bifunctor<'a, A, B> for (A, B) {
    type Target<T: 'a, U: 'a> = (T, U);

    fn bimap<C: 'a, D: 'a, L: 'a, R: 'a>(self, left: L, right: R) -> Self::Target<C, D>
    where
        L: Fn(A) -> C,
        R: Fn(B) -> D,
    {
        (left(self.0), right(self.1))
    }
}

impl<'a, A: 'a, B: 'a> BifunctorRef<'a, A, B> for (A, B) {
    fn bimap_ref<C: 'a, D: 'a, L: 'a, R: 'a>(&self, left: L, right: R) -> Self::Target<C, D>
    where
        L: Fn(&A) -> C,
        R: Fn(&B) -> D,
    {
        (left(&self.0), right(&self.1))
    }
}

impl<'a, A: 'a, B: 'a> Bifunctor<'a, A, B> for Vec<(A, B)> {
    type Target<T, U> = Vec<(T, U)> where T: 'a, U: 'a;

    fn bimap<C, D, L, R>(self, left: L, right: R) -> Self::Target<C, D>
    where
        C: 'a,
        D: 'a,
        L: Fn(A) -> C,
        R: Fn(B) -> D,
    {
        self.into_iter().map(|(a, b)| (left(a), right(b))).collect()
    }
}

impl<'a, A: 'a, B: 'a> BifunctorRef<'a, A, B> for Vec<(A, B)> {
    fn bimap_ref<C: 'a, D: 'a, L: 'a, R: 'a>(&self, left: L, right: R) -> Self::Target<C, D>
    where
        L: Fn(&A) -> C,
        R: Fn(&B) -> D,
    {
        self.iter().map(|(a, b)| (left(a), right(b))).collect()
    }
}

#[cfg(feature = "futures")]
impl<'a, A: 'a, B: 'a> Bifunctor<'a, A, B> for futures::future::Either<A, B> {
    type Target<T, U> = futures::future::Either<T, U> where T: 'a, U: 'a;

    fn bimap<C, D, L, R>(self, left: L, right: R) -> Self::Target<C, D>
    where
        C: 'a,
        D: 'a,
        L: Fn(A) -> C,
        R: Fn(B) -> D,
    {
        match self {
            futures::future::Either::Left(lval) => futures::future::Either::Left(left(lval)),
            futures::future::Either::Right(rval) => futures::future::Either::Right(right(rval)),
        }
    }
}
