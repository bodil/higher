use std::collections::{LinkedList, VecDeque};

use crate::{Ap, Functor};
use higher::{Lift, Lift3};

/// `Apply` takes an `F<Fn(A) -> B>` and applies it to an `F<A>` to produce an
/// `F<B>`.
pub trait Apply<A, F, B>: Functor<A, B> + Lift3<A, F, B>
where
    F: Fn(A) -> B,
{
    fn apply(self, f: <Self as Lift3<A, F, B>>::Target2) -> <Self as Lift<A, B>>::Target1;
}

impl<A, F, B> Apply<A, F, B> for Option<A>
where
    F: Fn(A) -> B,
{
    fn apply(self, f: <Self as Lift3<A, F, B>>::Target2) -> <Self as Lift<A, B>>::Target1 {
        self.and_then(|v| f.map(|f| f(v)))
    }
}

impl<A, F, B, E> Apply<A, F, B> for Result<A, E>
where
    F: Fn(A) -> B,
{
    fn apply(self, f: <Self as Lift3<A, F, B>>::Target2) -> <Self as Lift<A, B>>::Target1 {
        self.and_then(|v| f.map(|f| f(v)))
    }
}

impl<A, F, B> Apply<A, F, B> for Vec<A>
where
    A: Clone,
    F: Fn(A) -> B + Clone,
{
    fn apply(self, f: <Self as Lift3<A, F, B>>::Target2) -> <Self as Lift<A, B>>::Target1 {
        self.ap(f)
    }
}

impl<A, F, B> Apply<A, F, B> for VecDeque<A>
where
    A: Clone,
    F: Fn(A) -> B + Clone,
{
    fn apply(self, f: <Self as Lift3<A, F, B>>::Target2) -> <Self as Lift<A, B>>::Target1 {
        self.ap(f)
    }
}

impl<A, F, B> Apply<A, F, B> for LinkedList<A>
where
    A: Clone,
    F: Fn(A) -> B + Clone,
{
    fn apply(self, f: <Self as Lift3<A, F, B>>::Target2) -> <Self as Lift<A, B>>::Target1 {
        self.ap(f)
    }
}
