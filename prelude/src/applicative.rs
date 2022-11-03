use crate::{Apply, Functor, Pure};

/// An `Applicative` functor is anything which implements `Functor`, `Apply` and
/// `Pure`.
pub trait Applicative<A>: Functor<A> + Apply<A> + Pure<A> {}

impl<M, A> Applicative<A> for M where M: Functor<A> + Apply<A> + Pure<A> {}
