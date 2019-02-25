use crate::{Extend, Extract};

pub trait Comonad<A, B>: Extend<A, B> + Extract<A> {}

impl<W, A, B> Comonad<A, B> for W where W: Extend<A, B> + Extract<A> {}
