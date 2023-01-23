use crate::Alt;

pub trait Plus<'a, A: 'a>: Alt<'a, A> + Default {}

impl<'a, A: 'a, M> Plus<'a, A> for M where
    M: Alt<'a, A> + Extend<A> + IntoIterator<Item = A> + Default
{
}
