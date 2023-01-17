use core::convert::identity;

use crate::{
    apply::{apply_second, ApplyFn},
    Applicative, Apply, Bind, Functor, Monad, Monoid, Pure, Semiring,
};

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

pub fn traverse_<A, B, L, MB, MU, MF, F>(func: F, l: L) -> MU
where
    L: Foldable<A>,
    MB: Applicative<B> + Apply<B, Target<ApplyFn<B, ()>> = MF> + Apply<B, Target<()> = MU>,
    MU: Applicative<()>
        + Apply<(), Target<B> = MB>
        + Apply<(), Target<ApplyFn<B, ()>> = MF>
        + Functor<(), Target<ApplyFn<B, ()>> = MF>,
    MF: Apply<ApplyFn<B, ()>, Target<B> = MB>,
    F: Fn(A) -> MB,
{
    #[allow(clippy::unit_arg)]
    l.foldr(
        |x, y| apply_second(func(x), y),
        Pure::pure(Default::default()),
    )
}

pub fn sequence_<A, L, MA, MU, MF>(l: L) -> MU
where
    L: Foldable<MA>,
    MA: Applicative<A> + Apply<A, Target<()> = MU> + Apply<A, Target<ApplyFn<A, ()>> = MF>,
    MU: Applicative<()>
        + Apply<(), Target<A> = MA>
        + Apply<(), Target<ApplyFn<A, ()>> = MF>
        + Functor<(), Target<ApplyFn<A, ()>> = MF>,
    MF: Apply<ApplyFn<A, ()>> + Apply<ApplyFn<A, ()>, Target<A> = MA>,
{
    traverse_(identity, l)
}

pub fn sum<A, L>(l: L) -> A
where
    A: Semiring,
    L: Foldable<A>,
{
    l.foldl(|a, b| a.add(b), A::ZERO)
}

pub fn product<A, L>(l: L) -> A
where
    A: Semiring,
    L: Foldable<A>,
{
    l.foldl(|a, b| a.mul(b), A::ONE)
}

#[cfg(feature = "std")]
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

#[cfg(all(test, feature = "std"))]
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
