/// A `Contravariant` functor.
pub trait Contravariant<'a, A> {
    type Target<B>;
    fn contramap<B, F>(self, f: F) -> Self::Target<B>
    where
        F: Fn(B) -> A + 'a;
}
