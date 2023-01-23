use std::{
    collections::{LinkedList, VecDeque},
    convert::identity,
};

use crate::{
    algebras::HeytingAlgebra,
    apply::ApplyFn,
    monoid::{Conj, Disj},
    rings::Semiring,
    Alt, Applicative, Apply, Bind, Functor, Monoid, Plus, Pure,
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
    fn fold_map_ref<F, M>(&'a self, f: F) -> M
    where
        F: Fn(&'a A) -> M + 'a,
        M: Monoid + 'a;

    fn fold(self) -> A
    where
        Self: Sized,
        A: Monoid,
    {
        self.fold_map(identity)
    }

    fn traverse_unit<B, MB, MU, MF, F>(self, func: F) -> MU
    where
        Self: Sized,
        A: 'a,
        B: 'a,
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
        self.foldr(
            move |x, y| func(x).apply_second(y),
            Pure::pure(Default::default()),
        )
    }

    fn sequence_unit<B, MA, MU, MF>(self) -> MU
    where
        Self: Sized,
        B: 'a,
        A: Applicative<'a, B>
            + Apply<'a, B, Target<()> = MU>
            + Apply<'a, B, Target<ApplyFn<'a, B, ()>> = MF>
            + 'a,
        MU: Applicative<'a, ()>
            + Apply<'a, (), Target<B> = A>
            + Apply<'a, (), Target<ApplyFn<'a, B, ()>> = MF>
            + Functor<'a, (), Target<ApplyFn<'a, B, ()>> = MF>
            + 'a,
        MF: Apply<'a, ApplyFn<'a, B, ()>> + Apply<'a, ApplyFn<'a, B, ()>, Target<B> = A>,
    {
        self.traverse_unit(identity)
    }

    fn fold_m<M, B, F>(&'a self, f: &'a F, init: B) -> M
    where
        Self: Sized,
        A: 'a,
        B: 'a,
        M: Pure<B> + Bind<'a, B, Target<B> = M> + 'a,
        F: Fn(B, &'a A) -> M + 'a,
    {
        self.foldl_ref(move |m, a| m.bind::<B, _>(move |b| f(b, a)), M::pure(init))
    }

    fn sum(self) -> A
    where
        Self: Sized,
        A: Semiring + 'a,
    {
        self.foldl(|a, b| a.add(b), A::ZERO)
    }

    fn product(self) -> A
    where
        Self: Sized,
        A: Semiring + 'a,
    {
        self.foldl(|a, b| a.mul(b), A::ONE)
    }

    fn one_of<B: 'a>(self) -> A
    where
        Self: Sized,
        A: Plus<'a, B>,
    {
        self.foldr(Alt::alt, Default::default())
    }

    fn one_of_map<B: 'a, GB: 'a, F: 'a>(self, f: F) -> GB
    where
        Self: Sized,
        GB: Plus<'a, B>,
        F: Fn(A) -> GB,
    {
        self.foldr(move |a, bs| f(a).alt(bs), Default::default())
    }

    fn all<B: 'a, F: 'a>(&'a self, f: F) -> B
    where
        Self: Sized,
        F: Fn(&'a A) -> B,
        B: HeytingAlgebra,
    {
        self.fold_map_ref(move |a| Conj(f(a))).unwrap()
    }

    fn any<B: 'a, F: 'a>(&'a self, f: F) -> B
    where
        Self: Sized,
        F: Fn(&'a A) -> B,
        B: HeytingAlgebra,
    {
        self.fold_map_ref(move |a| Disj(f(a))).unwrap()
    }

    fn contains(&'a self, value: &'a A) -> bool
    where
        Self: Sized,
        A: Eq,
    {
        self.any(move |item| item == value)
    }
}

/// A default implementation for [`fold_map`](Foldable::fold_map) using
/// [`foldl`](Foldable::foldl).
pub fn fold_map_default_l<'a, A, L, M, F>(f: F, l: L) -> M
where
    A: 'a,
    L: Foldable<'a, A>,
    M: Monoid + 'a,
    F: Fn(A) -> M + 'a,
{
    l.foldl(move |acc, x| acc.mappend(f(x)), Default::default())
}

/// A default implementation for [`fold_map_ref`](Foldable::fold_map_ref) using
/// [`foldl_ref`](Foldable::foldl_ref).
pub fn fold_map_default_l_ref<'a, A, L, M, F>(f: F, l: &'a L) -> M
where
    A: 'a,
    L: Foldable<'a, A>,
    M: Monoid + 'a,
    F: Fn(&'a A) -> M + 'a,
{
    l.foldl_ref(move |acc, x| acc.mappend(f(x)), Default::default())
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

    fn fold_map_ref<F, M>(&'a self, f: F) -> M
    where
        F: Fn(&'a A) -> M + 'a,
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

    fn fold_map_ref<F, M>(&'a self, f: F) -> M
    where
        F: Fn(&'a A) -> M + 'a,
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

        fn fold_map_ref<F, M>(&'a self, f: F) -> M
        where
            F: Fn(&'a A) -> M + 'a,
            M: Monoid + 'a,
        {
            fold_map_default_l_ref(f, self)
        }
    };
}

impl<'a, A: 'a, const N: usize> Foldable<'a, A> for [A; N] {
    impl_foldable_from_iter!();
}

impl<'a, A: 'a> Foldable<'a, A> for Vec<A> {
    impl_foldable_from_iter!();
}

impl<'a, A: 'a> Foldable<'a, A> for VecDeque<A> {
    impl_foldable_from_iter!();
}

impl<'a, A: 'a> Foldable<'a, A> for LinkedList<A> {
    impl_foldable_from_iter!();
}

#[cfg(test)]
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
