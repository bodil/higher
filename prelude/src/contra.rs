/// A `Contravariant` functor.
pub trait Contravariant<'a, A: 'a> {
    type Target<T: 'a>;

    fn contramap<B: 'a, F: 'a>(self, f: F) -> Self::Target<B>
    where
        F: Fn(B) -> A;
}
