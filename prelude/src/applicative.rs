use crate::{Apply, Functor, Pure};

/// An `Applicative` functor is anything which implements [`Functor`](Functor),
/// [`Apply`](Apply) and [`Pure`](Pure).
pub trait Applicative<'a, A>: Functor<'a, A> + Apply<'a, A> + Pure<A>
where
    A: 'a,
{
}

impl<'a, M, A: 'a> Applicative<'a, A> for M where M: Functor<'a, A> + Apply<'a, A> + Pure<A> {}
