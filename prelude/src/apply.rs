use crate::Functor;

/// `Apply` takes an `F<Fn(A) -> B>` and applies it to an `F<A>` to produce an
/// `F<B>`.
pub trait Apply<A>: Functor<A> {
    type Target<T>;
    fn apply<B, F>(self, f: <Self as Apply<A>>::Target<F>) -> <Self as Apply<A>>::Target<B>
    where
        F: Fn(A) -> B;
}

impl<A> Apply<A> for Option<A> {
    type Target<T> = Option<T>;

    fn apply<B, F>(self, f: <Self as Apply<A>>::Target<F>) -> <Self as Apply<A>>::Target<B>
    where
        F: Fn(A) -> B,
    {
        self.and_then(|x| f.map(|f| f(x)))
    }
}

impl<A, E> Apply<A> for Result<A, E> {
    type Target<T> = Result<T, E>;

    fn apply<B, F>(self, f: <Self as Apply<A>>::Target<F>) -> <Self as Apply<A>>::Target<B>
    where
        F: Fn(A) -> B,
    {
        self.and_then(|x| f.map(|f| f(x)))
    }
}

#[cfg(feature = "std")]
impl<A> Apply<A> for Vec<A>
where
    Vec<A>: Clone,
{
    type Target<T> = Vec<T>;

    fn apply<B, F>(self, f: <Self as Apply<A>>::Target<F>) -> <Self as Apply<A>>::Target<B>
    where
        F: Fn(A) -> B,
    {
        crate::ap(f, self)
    }
}
