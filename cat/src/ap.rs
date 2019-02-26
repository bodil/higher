use crate::{Bind, Pure};
use higher::{Lift, Lift3};

/// `Ap` provides an implementation for `Apply::apply` using only `Bind` and
/// `Pure`.
pub trait Ap<A, F, B>: Lift3<A, F, B> + Bind<A, B>
where
    A: Clone,
    <Self as Lift3<A, F, B>>::Target2: Bind<F, B> + Clone,
    <<Self as Lift3<A, F, B>>::Target2 as Lift<F, B>>::Target1: Pure<B>,
    F: Fn(A) -> B,
{
    fn ap(self, f: <Self as Lift3<A, F, B>>::Target2) -> <Self as Lift<A, B>>::Target1;
}

impl<M, A, F, B> Ap<A, F, B> for M
where
    M: Lift3<A, F, B> + Bind<A, B>,
    A: Clone,
    <M as Lift3<A, F, B>>::Target2: Bind<F, B> + Clone,
    <<M as Lift3<A, F, B>>::Target2 as Lift<F, B>>::Target1: Pure<B>,
    F: Fn(A) -> B,
{
    fn ap(self, f: <Self as Lift3<A, F, B>>::Target2) -> <Self as Lift<A, B>>::Target1 {
        self.bind(|v: A| {
            let m: <<Self as Lift3<A, F, B>>::Target2 as Lift<F, B>>::Target1 =
                f.clone().bind(|fun: F| Pure::pure(fun(v.clone())));
            Self::cast(m)
        })
    }
}
