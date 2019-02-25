use crate::Lift;

pub trait Bind<A, B>: Lift<A, B> {
    /// Use the value inside an `M<A>: Bind` to create an `M<B>: Bind`.
    fn bind<F>(self, f: F) -> <Self as Lift<A, B>>::Target1
    where
        F: Fn(A) -> <Self as Lift<A, B>>::Target1;
}

impl<A, B> Bind<A, B> for Option<A> {
    fn bind<F>(self, f: F) -> <Self as Lift<A, B>>::Target1
    where
        F: Fn(A) -> <Self as Lift<A, B>>::Target1,
    {
        self.and_then(f)
    }
}

impl<A, B, E> Bind<A, B> for Result<A, E> {
    fn bind<F>(self, f: F) -> <Self as Lift<A, B>>::Target1
    where
        F: Fn(A) -> <Self as Lift<A, B>>::Target1,
    {
        self.and_then(f)
    }
}

impl<A, B> Bind<A, B> for Vec<A> {
    fn bind<F>(self, f: F) -> <Self as Lift<A, B>>::Target1
    where
        F: Fn(A) -> <Self as Lift<A, B>>::Target1,
    {
        self.into_iter().flat_map(|v| f(v).into_iter()).collect()
    }
}
