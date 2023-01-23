use crate::Functor;

pub trait Alt<'a, A: 'a>: Functor<'a, A> {
    fn alt(self, other: Self) -> Self;
}

impl<'a, A: 'a, M> Alt<'a, A> for M
where
    M: Functor<'a, A> + Extend<A> + IntoIterator<Item = A>,
{
    fn alt(mut self, other: Self) -> Self {
        self.extend(other.into_iter());
        self
    }
}
