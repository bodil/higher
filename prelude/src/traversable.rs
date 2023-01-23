use std::{
    collections::{LinkedList, VecDeque},
    convert::identity,
    iter,
};

use crate::{
    apply::{lift2, ApplyFn},
    Applicative, Foldable, Functor, Pure,
};

pub trait Traversable<'a, A: 'a>: Functor<'a, A> + Foldable<'a, A> {
    fn traverse<B: 'a, M: 'a, F: 'a>(self, f: F) -> M::Target<Self::Target<B>>
    where
        Self::Target<B>: Traversable<'a, B>,
        B: Clone,
        M: Applicative<'a, B>,

        M::Target<Self::Target<B>>: Applicative<'a, Self::Target<B>, Target<B> = M>
            + Applicative<'a, Self::Target<B>, Target<Self::Target<B>> = M::Target<Self::Target<B>>>
            + Applicative<
                'a,
                Self::Target<B>,
                Target<ApplyFn<'a, B, Self::Target<B>>> = M::Target<
                    ApplyFn<'a, B, Self::Target<B>>,
                >,
            >,
        M::Target<ApplyFn<'a, B, Self::Target<B>>>: Applicative<
            'a,
            ApplyFn<'a, B, Self::Target<B>>,
            Target<Self::Target<B>> = M::Target<Self::Target<B>>,
        >,
        F: Fn(A) -> M;

    fn sequence<B: 'a>(self) -> A::Target<Self::Target<B>>
    where
        Self: Sized,
        Self::Target<B>: Traversable<'a, B>,
        A: Applicative<'a, B>,
        B: Clone,

        A::Target<Self::Target<B>>: Applicative<'a, Self::Target<B>, Target<B> = A>
            + Applicative<'a, Self::Target<B>, Target<Self::Target<B>> = A::Target<Self::Target<B>>>
            + Applicative<
                'a,
                Self::Target<B>,
                Target<ApplyFn<'a, B, Self::Target<B>>> = A::Target<
                    ApplyFn<'a, B, Self::Target<B>>,
                >,
            >,
        A::Target<ApplyFn<'a, B, Self::Target<B>>>: Applicative<
            'a,
            ApplyFn<'a, B, Self::Target<B>>,
            Target<Self::Target<B>> = A::Target<Self::Target<B>>,
        >,
    {
        self.traverse(identity)
    }
}

impl<'a, A: 'a> Traversable<'a, A> for Option<A> {
    fn traverse<B: 'a, M, F: 'a>(self, f: F) -> M::Target<Self::Target<B>>
    where
        Self::Target<B>: Traversable<'a, B>,
        M: Applicative<'a, B>,

        M::Target<Self::Target<B>>: Applicative<'a, Self::Target<B>, Target<B> = M>
            + Applicative<'a, Self::Target<B>>
            + Applicative<'a, Self::Target<B>, Target<Self::Target<B>> = M::Target<Self::Target<B>>>,
        M::Target<ApplyFn<'a, B, Self::Target<B>>>:
            Applicative<'a, ApplyFn<'a, B, Self::Target<B>>>,
        F: Fn(A) -> M,
    {
        match self {
            None => Pure::pure(None),
            Some(a) => f(a).fmap(Some),
        }
    }
}

impl<'a, A: 'a, E: 'a> Traversable<'a, A> for Result<A, E> {
    fn traverse<B: 'a, M, F: 'a>(self, f: F) -> M::Target<Self::Target<B>>
    where
        Self::Target<B>: Traversable<'a, B>,
        M: Applicative<'a, B>,

        M::Target<Self::Target<B>>: Applicative<'a, Self::Target<B>, Target<B> = M>,
        M::Target<ApplyFn<'a, B, Self::Target<B>>>:
            Applicative<'a, ApplyFn<'a, B, Self::Target<B>>>,
        F: Fn(A) -> M,
    {
        match self {
            Err(e) => Pure::pure(Err(e)),
            Ok(a) => f(a).fmap(Ok),
        }
    }
}

/// A default implementation of [`traverse`](traverse) for anything that
/// implements [`Foldable`](Foldable), [`Default`](Default) and
/// [`Extend`](Extend).
pub fn traverse_extend<'a, A: 'a, B: 'a, L: 'a, M: 'a, F: 'a>(f: F, l: L) -> M::Target<L::Target<B>>
where
    L: Traversable<'a, A>,
    L::Target<B>: Foldable<'a, B> + Extend<B> + Default + Clone,
    M: Applicative<'a, B>,

    M::Target<L::Target<B>>: Applicative<'a, L::Target<B>, Target<B> = M>
        + Applicative<'a, L::Target<B>, Target<L::Target<B>> = M::Target<L::Target<B>>>
        + Applicative<
            'a,
            L::Target<B>,
            Target<ApplyFn<'a, B, L::Target<B>>> = M::Target<ApplyFn<'a, B, L::Target<B>>>,
        >,
    M::Target<ApplyFn<'a, B, L::Target<B>>>: Applicative<
        'a,
        ApplyFn<'a, B, L::Target<B>>,
        Target<L::Target<B>> = M::Target<L::Target<B>>,
    >,
    F: Fn(A) -> M,
{
    fn snoc<L: Extend<A>, A>(mut l: L, a: A) -> L {
        l.extend(iter::once(a));
        l
    }

    l.foldl(
        move |ys, x| lift2(&snoc, ys, f(x)),
        Pure::pure(Default::default()),
    )
}

macro_rules! impl_traversable_for_extendable {
    ($type:ident) => {
        impl<'a, A: 'a> Traversable<'a, A> for $type<A> {
            fn traverse<B: 'a, M: 'a, F: 'a>(self, f: F) -> M::Target<Self::Target<B>>
            where
                Self::Target<B>: Foldable<'a, B>,
                M: Applicative<'a, B>,
                B: Clone,

                M::Target<Self::Target<B>>: Applicative<'a, Self::Target<B>, Target<B> = M>
                    + Applicative<
                        'a,
                        Self::Target<B>,
                        Target<Self::Target<B>> = M::Target<Self::Target<B>>,
                    > + Applicative<
                        'a,
                        Self::Target<B>,
                        Target<ApplyFn<'a, B, Self::Target<B>>> = M::Target<
                            ApplyFn<'a, B, Self::Target<B>>,
                        >,
                    >,
                M::Target<ApplyFn<'a, B, Self::Target<B>>>: Applicative<
                    'a,
                    ApplyFn<'a, B, Self::Target<B>>,
                    Target<Self::Target<B>> = M::Target<Self::Target<B>>,
                >,
                F: Fn(A) -> M,
            {
                traverse_extend(f, self)
            }
        }
    };
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
