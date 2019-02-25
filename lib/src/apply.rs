use crate::{Bind, Functor, Lift, Lift3};

pub trait Apply<A, B, C>: Functor<A, C> + Lift3<A, B, C>
where
    B: Fn(A) -> C,
{
    fn apply(self, f: <Self as Lift3<A, B, C>>::Target2) -> <Self as Lift<A, C>>::Target1;
}

impl<A, B, C> Apply<A, B, C> for Option<A>
where
    B: Fn(A) -> C,
{
    fn apply(self, f: <Self as Lift3<A, B, C>>::Target2) -> <Self as Lift<A, C>>::Target1 {
        self.and_then(|v| f.map(|f| f(v)))
    }
}

impl<A, B, C, E> Apply<A, B, C> for Result<A, E>
where
    B: Fn(A) -> C,
{
    fn apply(self, f: <Self as Lift3<A, B, C>>::Target2) -> <Self as Lift<A, C>>::Target1 {
        self.and_then(|v| f.map(|f| f(v)))
    }
}

impl<A, B, C> Apply<A, B, C> for Vec<A>
where
    A: Clone,
    B: Fn(A) -> C + Clone,
{
    fn apply(self, f: <Self as Lift3<A, B, C>>::Target2) -> <Self as Lift<A, C>>::Target1 {
        self.bind(|v: A| f.clone().map(|f2| f2(v.clone())))
    }
}
