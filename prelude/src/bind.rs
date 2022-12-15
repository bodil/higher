use crate::Pure;

/// `Bind` lets you chain computations together.
///
/// It takes a function `Fn(A) -> M<B>` and applies it to the `A` inside `M<A>`.
/// You can think of this as a callback function for when the value of `A` is
/// ready to be processed, returning the next computation in the sequence.
pub trait Bind<A> {
    type Target<T>;
    fn bind<B, F>(self, f: F) -> Self::Target<B>
    where
        F: Fn(A) -> Self::Target<B>;
}

/// `LiftM1` provides a default implementation for `Functor::map` using
/// only `Bind` and `Pure`.
pub fn lift_m1<MA, MB, A, B, F>(f: F, a: MA) -> MB
where
    F: Fn(A) -> B,
    MA: Bind<A, Target<B> = MB>,
    MB: Pure<B>,
{
    a.bind::<B, _>(|x| MB::pure(f(x)))
}

impl<A> Bind<A> for Option<A> {
    type Target<T> = Option<T>;

    fn bind<B, F>(self, f: F) -> Self::Target<B>
    where
        F: Fn(A) -> Self::Target<B>,
    {
        self.and_then(f)
    }
}

impl<A, E> Bind<A> for Result<A, E> {
    type Target<T> = Result<T, E>;

    fn bind<B, F>(self, f: F) -> Self::Target<B>
    where
        F: Fn(A) -> Self::Target<B>,
    {
        self.and_then(f)
    }
}

#[cfg(feature = "std")]
impl<A> Bind<A> for Vec<A> {
    type Target<T> = Vec<T>;

    fn bind<B, F>(self, f: F) -> Self::Target<B>
    where
        F: Fn(A) -> Self::Target<B>,
    {
        self.into_iter().flat_map(|v| f(v).into_iter()).collect()
    }
}

#[cfg(test)]
mod test {
    use crate::Bind;

    #[test]
    fn bind_vec() {
        let v = vec![1, 2, 3];
        let o = v.bind(|i| vec![i, i + 1]);
        assert_eq!(vec![1, 2, 2, 3, 3, 4], o);
    }
}
