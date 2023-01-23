use crate::{Applicative, Plus};

pub trait Alternative<'a, A: 'a>: Applicative<'a, A> + Plus<'a, A> {}

impl<'a, A: 'a, M> Alternative<'a, A> for M where M: Applicative<'a, A> + Plus<'a, A> {}
