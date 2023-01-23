use std::{
    collections::{LinkedList, VecDeque},
    convert::identity,
    iter,
};

use crate::{
    apply::{lift2, ApplyFn},
    Apply, Foldable, Functor, Pure,
};

pub trait Traversable<'a, A: 'a>: Functor<'a, A> + Foldable<'a, A> {
    type Target<T: 'a>;

    fn traverse<B, MB, MLB, MF, F>(self, f: F) -> MLB
    where
        B: Clone + 'a,
        <Self as Traversable<'a, A>>::Target<B>: Foldable<'a, B> + Clone + 'a,
        MB: Apply<'a, B, Target<<Self as Traversable<'a, A>>::Target<B>> = MLB>
            + Functor<'a, B, Target<ApplyFn<'a, <Self as Traversable<'a, A>>::Target<B>, <Self as Traversable<'a, A>>::Target<B>>> = MF>
            + Functor<'a, B, Target<<Self as Traversable<'a, A>>::Target<B>> = MLB>,
        MLB: Apply<'a, <Self as Traversable<'a, A>>::Target<B>, Target<B> = MB>
            + Apply<'a, <Self as Traversable<'a, A>>::Target<B>, Target<ApplyFn<'a, <Self as Traversable<'a, A>>::Target<B>, <Self as Traversable<'a, A>>::Target<B>>> = MF>
            + Apply<'a,<Self as Traversable<'a, A>>::Target<B>,Target<<Self as Traversable<'a, A>>::Target<B>> = MLB>
            + Functor<'a, <Self as Traversable<'a, A>>::Target<B>, Target<<Self as Traversable<'a, A>>::Target<B>> = MLB>
            + Pure<<Self as Traversable<'a, A>>::Target<B>>
            + 'a,
        MF: Apply<'a, ApplyFn<'a, <Self as Traversable<'a, A>>::Target<B>, <Self as Traversable<'a, A>>::Target<B>>>,
        F: Fn(A) -> MB + 'a;

    fn sequence<B, MLB, MF>(self) -> MLB
    where
        Self: Sized,
        B: Clone+'a,
        <Self as Traversable<'a, A>>::Target<B>: Foldable<'a, B> + Clone + 'a,
        A: Apply<'a, B, Target<<Self as Traversable<'a, A>>::Target<B>> = MLB>
            + Functor<'a, B, Target<ApplyFn<'a, <Self as Traversable<'a, A>>::Target<B>, <Self as Traversable<'a, A>>::Target<B>>> = MF>
            + Functor<'a, B, Target<<Self as Traversable<'a, A>>::Target<B>> = MLB>,
        MLB: Apply<'a, <Self as Traversable<'a, A>>::Target<B>, Target<B> = A>
            + Apply<'a, <Self as Traversable<'a, A>>::Target<B>, Target<ApplyFn<'a, <Self as Traversable<'a, A>>::Target<B>, <Self as Traversable<'a, A>>::Target<B>>> = MF>
            + Apply<'a,<Self as Traversable<'a, A>>::Target<B>,Target<<Self as Traversable<'a, A>>::Target<B>> = MLB>
            + Functor<'a, <Self as Traversable<'a, A>>::Target<B>, Target<<Self as Traversable<'a, A>>::Target<B>> = MLB>
            + Pure<<Self as Traversable<'a, A>>::Target<B>>
            + 'a,
        MF: Apply<'a, ApplyFn<'a, <Self as Traversable<'a, A>>::Target<B>, <Self as Traversable<'a, A>>::Target<B>>>,
    {
        self.traverse(identity)
    }
}

impl<'a, A: 'a> Traversable<'a, A> for Option<A> {
    type Target<T: 'a> = Option<T>;

    fn traverse<B, MB, MLB, MF, F>(self, f: F) -> MLB
    where
        B: Clone + 'a,
        <Self as Traversable<'a, A>>::Target<B>: Foldable<'a, B> + Clone + 'a,
        MB: Apply<'a, B, Target<<Self as Traversable<'a, A>>::Target<B>> = MLB>
            + Functor<'a, B, Target<ApplyFn<'a, <Self as Traversable<'a, A>>::Target<B>, <Self as Traversable<'a, A>>::Target<B>>> = MF>
            + Functor<'a, B, Target<<Self as Traversable<'a, A>>::Target<B>> = MLB>,
        MLB: Apply<'a, <Self as Traversable<'a, A>>::Target<B>, Target<B> = MB>
            + Apply<'a, <Self as Traversable<'a, A>>::Target<B>, Target<ApplyFn<'a, <Self as Traversable<'a, A>>::Target<B>, <Self as Traversable<'a, A>>::Target<B>>> = MF>
            + Apply<'a,<Self as Traversable<'a, A>>::Target<B>,Target<<Self as Traversable<'a, A>>::Target<B>> = MLB>
            + Functor<'a, <Self as Traversable<'a, A>>::Target<B>, Target<<Self as Traversable<'a, A>>::Target<B>> = MLB>
            + Pure<<Self as Traversable<'a, A>>::Target<B>>
            + 'a,
        MF: Apply<'a, ApplyFn<'a, <Self as Traversable<'a, A>>::Target<B>, <Self as Traversable<'a, A>>::Target<B>>>,
        F: Fn(A) -> MB + 'a,
    {
        match self {
            None => Pure::pure(None),
            Some(a) => f(a).fmap(Some),
        }
    }
}

impl<'a, A: 'a, E: 'a> Traversable<'a, A> for Result<A, E> {
    type Target<T: 'a> = Result<T, E>;

    fn traverse<B, MB, MLB, MF, F>(self, f: F) -> MLB
    where
        B: Clone + 'a,
        <Self as Traversable<'a, A>>::Target<B>: Foldable<'a, B> + Clone + 'a,
        MB: Apply<'a, B, Target<<Self as Traversable<'a, A>>::Target<B>> = MLB>
            + Functor<'a, B, Target<ApplyFn<'a, <Self as Traversable<'a, A>>::Target<B>, <Self as Traversable<'a, A>>::Target<B>>> = MF>
            + Functor<'a, B, Target<<Self as Traversable<'a, A>>::Target<B>> = MLB>,
        MLB: Apply<'a, <Self as Traversable<'a, A>>::Target<B>, Target<B> = MB>
            + Apply<'a, <Self as Traversable<'a, A>>::Target<B>, Target<ApplyFn<'a, <Self as Traversable<'a, A>>::Target<B>, <Self as Traversable<'a, A>>::Target<B>>> = MF>
            + Apply<'a,<Self as Traversable<'a, A>>::Target<B>,Target<<Self as Traversable<'a, A>>::Target<B>> = MLB>
            + Functor<'a, <Self as Traversable<'a, A>>::Target<B>, Target<<Self as Traversable<'a, A>>::Target<B>> = MLB>
            + Pure<<Self as Traversable<'a, A>>::Target<B>>
            + 'a,
        MF: Apply<'a, ApplyFn<'a, <Self as Traversable<'a, A>>::Target<B>, <Self as Traversable<'a, A>>::Target<B>>>,
        F: Fn(A) -> MB + 'a,
    {
        match self {
            Err(e) => Pure::pure(Err(e)),
            Ok(a) => f(a).fmap(Ok),
        }
    }
}

/// An implementation of [`traverse`](traverse) for anything that implements
/// [`Foldable`](Foldable), [`Default`](Default) and [`Extend`](Extend).
pub fn traverse_extend<'a, A, B, LA, LB, MB, MLB, MF, F>(f: F, l: LA) -> MLB
where
    A: 'a,
    B: Clone + 'a,
    LA: Foldable<'a, A>,
    LB: Foldable<'a, B> + Extend<B> + Default + Clone + 'a,
    MB: Apply<'a, B, Target<LB> = MLB> + Functor<'a, B, Target<ApplyFn<'a, LB, LB>> = MF>,
    MLB: Apply<'a, LB, Target<B> = MB>
        + Apply<'a, LB, Target<ApplyFn<'a, LB, LB>> = MF>
        + Apply<'a, LB, Target<LB> = MLB>
        + Functor<'a, LB, Target<LB> = MLB>
        + Pure<LB>
        + 'a,
    MF: Apply<'a, ApplyFn<'a, LB, LB>>,
    F: Fn(A) -> MB + 'a,
{
    let out: MLB = Pure::pure(Default::default());
    let cons_f = move |ys: MLB, x: A| {
        lift2(
            |item: B, mut list: LB| {
                list.extend(iter::once(item));
                list
            },
            f(x),
            ys,
        )
    };
    l.foldl(cons_f, out)
}

macro_rules! impl_traversable_for_extendable {
    ($type:ident) => {
        impl<'a, A: 'a> Traversable<'a, A> for $type<A> {
            type Target<T: 'a> = $type<T>;

            fn traverse<B,  MB, MLB, MF, F>(self, f: F) -> MLB
            where
            B: Clone + 'a,
            <Self as Traversable<'a, A>>::Target<B>: Foldable<'a, B> + Clone + 'a,
            MB: Apply<'a, B, Target<<Self as Traversable<'a, A>>::Target<B>> = MLB> + Functor<'a, B, Target<ApplyFn<'a, <Self as Traversable<'a, A>>::Target<B>, <Self as Traversable<'a, A>>::Target<B>>> = MF>,
            MLB: Apply<'a, <Self as Traversable<'a, A>>::Target<B>, Target<B> = MB>
            + Apply<'a, <Self as Traversable<'a, A>>::Target<B>, Target<ApplyFn<'a, <Self as Traversable<'a, A>>::Target<B>, <Self as Traversable<'a, A>>::Target<B>>> = MF>
            + Apply<'a,<Self as Traversable<'a, A>>::Target<B>,Target<<Self as Traversable<'a, A>>::Target<B>> = MLB>
            + Functor<'a, <Self as Traversable<'a, A>>::Target<B>, Target<<Self as Traversable<'a, A>>::Target<B>> = MLB>
            + Pure<<Self as Traversable<'a, A>>::Target<B>>
            + 'a,
            MF: Apply<'a, ApplyFn<'a, <Self as Traversable<'a, A>>::Target<B>, <Self as Traversable<'a, A>>::Target<B>>>,
            F: Fn(A) -> MB + 'a,
            {
                traverse_extend(f, self)
            }
        }
    }
}

impl_traversable_for_extendable!(Vec);
impl_traversable_for_extendable!(VecDeque);
impl_traversable_for_extendable!(LinkedList);

#[cfg(test)]
mod test {
    use super::Traversable;

    #[test]
    fn sequence_option() {
        let m: Vec<Option<i32>> = vec![Some(1), Some(2), Some(3)];
        let n: Option<Vec<i32>> = m.sequence();
        assert_eq!(n, Some(vec![1, 2, 3]));
    }

    #[test]
    fn traverse_option() {
        let m: Vec<i32> = vec![1, 2, 3];
        let n: Option<Vec<i32>> = m.traverse(|a| Some(a * 2));
        assert_eq!(n, Some(vec![2, 4, 6]));
    }
}
