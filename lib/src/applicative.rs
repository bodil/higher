use crate::{Apply, Functor, Pure};

pub trait Applicative<A, F, B>: Functor<A, B> + Apply<A, F, B> + Pure<A>
where
    F: Fn(A) -> B,
{
}

impl<M, A, F, B> Applicative<A, F, B> for M
where
    M: Functor<A, B> + Apply<A, F, B> + Pure<A>,
    F: Fn(A) -> B,
{
}
