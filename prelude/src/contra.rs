/// A `Contravariant` functor.
pub trait Contravariant<'a, A>
where
    A: 'a,
{
    type Target<T>
    where
        T: 'a;

    fn contramap<B, F>(self, f: F) -> Self::Target<B>
    where
        B: 'a,
        F: Fn(B) -> A + 'a;
}
