use std::convert::identity;

use crate::{
    apply::{apply_second, ApplyFn},
    rings::Semiring,
    Applicative, Apply, Bind, Functor, Monoid, Pure,
};

pub trait Foldable<'a, A>
where
    A: 'a,
{
    fn foldr<B, F>(self, f: F, init: B) -> B
    where
        B: 'a,
        F: Fn(A, B) -> B + 'a;
    fn foldr_ref<B, F>(&'a self, f: F, init: B) -> B
    where
        B: 'a,
        F: Fn(&'a A, B) -> B + 'a;
    fn foldl<B, F>(self, f: F, init: B) -> B
    where
        B: 'a,
        F: Fn(B, A) -> B + 'a;
    fn foldl_ref<B, F>(&'a self, f: F, init: B) -> B
    where
        B: 'a,
        F: Fn(B, &'a A) -> B + 'a;
    fn fold_map<F, M>(self, f: F) -> M
    where
        F: Fn(A) -> M + 'a,
        M: Monoid;
}

pub fn fold<'a, L, A>(l: L) -> A
where
    L: Foldable<'a, A>,
    A: Monoid + 'a,
{
    l.fold_map(identity)
}

pub fn fold_m<'a, M, L, A, B, F>(f: &'a F, init: B, l: &'a L) -> M
where
    A: 'a,
    B: 'a,
    L: Foldable<'a, A> + 'a,
    M: Pure<B> + Bind<'a, B, Target<B> = M> + 'a,
    F: Fn(B, &'a A) -> M + 'a,
{
    l.foldl_ref(move |m, a| m.bind::<B, _>(move |b| f(b, a)), M::pure(init))
}

pub fn fold_map_default_l<'a, A, L, M, F>(f: F, l: L) -> M
where
    A: 'a,
    L: Foldable<'a, A>,
    M: Monoid + 'a,
    F: Fn(A) -> M + 'a,
{
    l.foldl(move |acc, x| acc.mappend(f(x)), Default::default())
}

pub fn traverse_<'a, A, B, L, MB, MU, MF, F>(func: F, l: L) -> MU
where
    A: 'a,
    B: 'a,
    L: Foldable<'a, A>,
    MB: Applicative<'a, B>
        + Apply<'a, B, Target<ApplyFn<'a, B, ()>> = MF>
        + Apply<'a, B, Target<()> = MU>,
    MU: Applicative<'a, ()>
        + Apply<'a, (), Target<B> = MB>
        + Apply<'a, (), Target<ApplyFn<'a, B, ()>> = MF>
        + Functor<'a, (), Target<ApplyFn<'a, B, ()>> = MF>
        + 'a,
    MF: Apply<'a, ApplyFn<'a, B, ()>, Target<B> = MB>,
    F: Fn(A) -> MB + 'a,
{
    #[allow(clippy::unit_arg)]
    l.foldr(
        move |x, y| apply_second(func(x), y),
        Pure::pure(Default::default()),
    )
}

pub fn sequence_<'a, A, L, MA, MU, MF>(l: L) -> MU
where
    A: 'a,
    L: Foldable<'a, MA>,
    MA: Applicative<'a, A>
        + Apply<'a, A, Target<()> = MU>
        + Apply<'a, A, Target<ApplyFn<'a, A, ()>> = MF>
        + 'a,
    MU: Applicative<'a, ()>
        + Apply<'a, (), Target<A> = MA>
        + Apply<'a, (), Target<ApplyFn<'a, A, ()>> = MF>
        + Functor<'a, (), Target<ApplyFn<'a, A, ()>> = MF>
        + 'a,
    MF: Apply<'a, ApplyFn<'a, A, ()>> + Apply<'a, ApplyFn<'a, A, ()>, Target<A> = MA>,
{
    traverse_(identity, l)
}

pub fn sum<'a, A, L>(l: L) -> A
where
    A: Semiring + 'a,
    L: Foldable<'a, A>,
{
    l.foldl(|a, b| a.add(b), A::ZERO)
}

pub fn product<'a, A, L>(l: L) -> A
where
    A: Semiring + 'a,
    L: Foldable<'a, A>,
{
    l.foldl(|a, b| a.mul(b), A::ONE)
}

// # Implementations

impl<'a, A: 'a> Foldable<'a, A> for Option<A> {
    fn foldr<B, F>(self, f: F, init: B) -> B
    where
        B: 'a,
        F: Fn(A, B) -> B + 'a,
    {
        match self {
            Some(value) => f(value, init),
            None => init,
        }
    }

    fn foldr_ref<B, F>(&'a self, f: F, init: B) -> B
    where
        B: 'a,
        F: Fn(&'a A, B) -> B + 'a,
    {
        match self {
            Some(value) => f(value, init),
            None => init,
        }
    }

    fn foldl<B, F>(self, f: F, init: B) -> B
    where
        B: 'a,
        F: Fn(B, A) -> B + 'a,
    {
        match self {
            Some(value) => f(init, value),
            None => init,
        }
    }

    fn foldl_ref<B, F>(&'a self, f: F, init: B) -> B
    where
        B: 'a,
        F: Fn(B, &'a A) -> B + 'a,
    {
        match self {
            Some(value) => f(init, value),
            None => init,
        }
    }

    fn fold_map<F, M>(self, f: F) -> M
    where
        F: Fn(A) -> M + 'a,
        M: Monoid,
    {
        match self {
            Some(value) => f(value),
            None => M::default(),
        }
    }
}

impl<'a, A: 'a, E> Foldable<'a, A> for Result<A, E> {
    fn foldr<B, F>(self, f: F, init: B) -> B
    where
        B: 'a,
        F: Fn(A, B) -> B + 'a,
    {
        match self {
            Ok(value) => f(value, init),
            Err(_) => init,
        }
    }

    fn foldr_ref<B, F>(&'a self, f: F, init: B) -> B
    where
        B: 'a,
        F: Fn(&'a A, B) -> B + 'a,
    {
        match self {
            Ok(value) => f(value, init),
            Err(_) => init,
        }
    }

    fn foldl<B, F>(self, f: F, init: B) -> B
    where
        B: 'a,
        F: Fn(B, A) -> B + 'a,
    {
        match self {
            Ok(value) => f(init, value),
            Err(_) => init,
        }
    }

    fn foldl_ref<B, F>(&'a self, f: F, init: B) -> B
    where
        B: 'a,
        F: Fn(B, &'a A) -> B + 'a,
    {
        match self {
            Ok(value) => f(init, value),
            Err(_) => init,
        }
    }

    fn fold_map<F, M>(self, f: F) -> M
    where
        F: Fn(A) -> M + 'a,
        M: Monoid,
    {
        match self {
            Ok(value) => f(value),
            Err(_) => M::default(),
        }
    }
}

macro_rules! impl_foldable_from_iter {
    () => {
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

        fn foldr_ref<B, F>(&'a self, f: F, init: B) -> B
        where
            A: 'a,
            F: Fn(&'a A, B) -> B + 'a,
        {
            self.iter().rfold(init, |a, b| f(b, a))
        }

        fn foldl_ref<B, F>(&'a self, f: F, init: B) -> B
        where
            A: 'a,
            F: Fn(B, &'a A) -> B + 'a,
        {
            self.iter().fold(init, f)
        }
    };
}

impl<'a, A: 'a, const N: usize> Foldable<'a, A> for [A; N] {
    impl_foldable_from_iter!();
}

impl<'a, A: 'a> Foldable<'a, A> for Vec<A> {
    impl_foldable_from_iter!();
}

impl<'a, A: 'a> Foldable<'a, A> for std::collections::VecDeque<A> {
    impl_foldable_from_iter!();
}

impl<'a, A: 'a> Foldable<'a, A> for std::collections::LinkedList<A> {
    impl_foldable_from_iter!();
}

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
        let b = a.fold_map(|x| x.to_string());
        assert_eq!(b, "12345");
    }
}
