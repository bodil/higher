use crate::{Alternative, Monad};

pub trait MonadPlus<'a, A: 'a>: Monad<'a, A> + Alternative<'a, A>
where
    A: Clone,
{
}

impl<'a, A: 'a, M> MonadPlus<'a, A> for M
where
    A: Clone,
    M: Monad<'a, A> + Alternative<'a, A>,
{
}
