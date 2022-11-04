use core::convert::identity;

use crate::{Bind, Monad, Monoid};

pub trait Foldable<A> {
    fn foldr<B, F>(self, f: F, init: B) -> B
    where
        F: Fn(A, B) -> B;
    fn foldl<B, F>(self, f: F, init: B) -> B
    where
        F: Fn(B, A) -> B;
    fn fold_map<F, M>(self, f: F) -> M
    where
        F: Fn(A) -> M,
        M: Monoid;
}

pub fn fold<L, A>(l: L) -> A
where
    L: Foldable<A>,
    A: Monoid,
{
    l.fold_map(identity)
}

pub fn fold_m<M, L, A, B, F>(f: F, init: B, l: L) -> M
where
    L: Foldable<A>,
    M: Monad<B> + Bind<B, Target<B> = M>,
    F: Fn(B, &A) -> M,
{
    l.foldl(|m, a| m.bind::<B, _>(|b| f(b, &a)), M::pure(init))
}

pub fn fold_map_default_l<A, L, M, F>(f: F, l: L) -> M
where
    L: Foldable<A>,
    M: Monoid,
    F: Fn(A) -> M,
{
    l.foldl(|acc, x| acc.mappend(f(x)), Default::default())
}

#[cfg(features = "std")]
impl<A> Foldable<A> for Vec<A> {
    fn foldl<B, F>(self, f: F, init: B) -> B
    where
        F: Fn(B, A) -> B,
    {
        self.into_iter().fold(init, f)
    }

    fn foldr<B, F>(self, f: F, init: B) -> B
    where
        F: Fn(A, B) -> B,
    {
        self.into_iter().rfold(init, |a, b| f(b, a))
    }

    fn fold_map<F, M>(self, f: F) -> M
    where
        F: Fn(A) -> M,
        M: Monoid,
    {
        fold_map_default_l(f, self)
    }
}

#[cfg(all(test, features = "std"))]
mod test {
    use crate::Foldable;

    #[test]
    fn foldl_vec() {
        let a = vec![1, 2, 3, 4, 5];
        let b = a.foldl(|acc, next| acc - next, 10);
        assert_eq!(b, -5);
    }

    #[test]
    fn foldr_vec() {
        let a = vec![1, 2, 3, 4, 5];
        let b = a.foldr(|acc, next| acc - next, 10);
        assert_eq!(b, -7);
    }

    #[test]
    fn foldmap_vec() {
        let a = vec![1, 2, 3, 4, 5];
        let b = a.fold_map(|x| x << 1);
        assert_eq!(b, 30);
    }
}
