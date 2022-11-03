use crate::{Extend, Extract};

/// A `Comonad` is the opposite of a `Monad`, and also anything which implements
/// `Extend` and `Extract`.
pub trait Comonad<A>: Extend<A> + Extract<A> {}

impl<W, A> Comonad<A> for W where W: Extend<A> + Extract<A> {}
