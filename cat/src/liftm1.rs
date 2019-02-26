use crate::{Bind, Pure};
use higher::Lift;

/// `LiftM1` provides a default implementation for `Functor::map` using
/// only `Bind` and `Pure`.
pub trait LiftM1<A, B>: Bind<A, B>
where
    <Self as Lift<A, B>>::Target1: Pure<B>,
{
    fn lift_m1<F>(self, f: F) -> <Self as Lift<A, B>>::Target1
    where
        F: Fn(A) -> B;
}

impl<M, A, B> LiftM1<A, B> for M
where
    M: Bind<A, B>,
    <M as Lift<A, B>>::Target1: Pure<B>,
{
    fn lift_m1<F>(self, f: F) -> <Self as Lift<A, B>>::Target1
    where
        F: Fn(A) -> B,
    {
        self.bind(|value| Pure::pure(f(value)))
    }
}
