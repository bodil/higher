use crate::{Applicative, Bind};

/// A `Monad` is like a burrito, and also anything which implements `Bind` and
/// `Applicative`.
pub trait Monad<A, F, B>: Bind<A, B> + Applicative<A, F, B>
where
    F: Fn(A) -> B,
{
}

impl<M, A, F, B> Monad<A, F, B> for M
where
    M: Bind<A, B> + Applicative<A, F, B>,
    F: Fn(A) -> B,
{
}
