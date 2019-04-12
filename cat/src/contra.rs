use higher::Lift;

/// A `Contravariant` functor.
pub trait Contravariant<A, B>: Lift<A, B> {
    fn contramap<F>(self, f: F) -> <Self as Lift<A, B>>::Target1
    where
        F: Fn(B) -> A;
}
