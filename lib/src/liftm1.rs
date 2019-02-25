use crate::{Bind, Lift, Pure};

pub trait LiftM1<A, B>: Bind<A, B> {
    fn lift_m1<F>(self, f: F) -> <Self as Lift<A, B>>::Target1
    where
        F: Fn(A) -> B,
        <Self as Lift<A, B>>::Target1: Pure<B>;
}

impl<M, A, B> LiftM1<A, B> for M
where
    M: Bind<A, B>,
{
    fn lift_m1<F>(self, f: F) -> <Self as Lift<A, B>>::Target1
    where
        F: Fn(A) -> B,
        <Self as Lift<A, B>>::Target1: Pure<B>,
    {
        self.bind(|value| Pure::pure(f(value)))
    }
}
