use crate::{Alternative, Monad};

pub trait MonadPlus<'a, A: 'a>: Monad<'a, A> + Alternative<'a, A> {}

impl<'a, A: 'a, M> MonadPlus<'a, A> for M where M: Monad<'a, A> + Alternative<'a, A> {}
