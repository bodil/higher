use crate::{Extend, Extract};

/// A `Comonad` is the opposite of a `Monad`, and also anything which implements
/// `Extend` and `Extract`.
pub trait Comonad<A, B>: Extend<A, B> + Extract<A> {}

impl<W, A, B> Comonad<A, B> for W where W: Extend<A, B> + Extract<A> {}
