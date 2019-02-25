use crate::{Bind, Lift, Lift3, Pure};

pub trait Ap<A, F, B>: Lift3<A, F, B> {
    fn ap(self, f: <Self as Lift3<A, F, B>>::Target2) -> <Self as Lift<A, B>>::Target1
    where
        A: Clone,
        Self: Bind<A, B>,
        <Self as Lift3<A, F, B>>::Target2: Bind<F, B> + Clone,
        <<Self as Lift3<A, F, B>>::Target2 as Lift<F, B>>::Target1: Pure<B>,
        F: Fn(A) -> B;
}

impl<M, A, F, B> Ap<A, F, B> for M
where
    M: Lift3<A, F, B>,
{
    fn ap(self, f: <Self as Lift3<A, F, B>>::Target2) -> <Self as Lift<A, B>>::Target1
    where
        A: Clone,
        Self: Bind<A, B>,
        <Self as Lift3<A, F, B>>::Target2: Bind<F, B> + Clone,
        <<Self as Lift3<A, F, B>>::Target2 as Lift<F, B>>::Target1: Pure<B>,
        F: Fn(A) -> B,
    {
        self.bind(|v: A| {
            let m: <<Self as Lift3<A, F, B>>::Target2 as Lift<F, B>>::Target1 =
                f.clone().bind(|fun: F| Pure::pure(fun(v.clone())));
            Self::cast(m)
        })
    }
}
