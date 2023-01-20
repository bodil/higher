use crate::Pure;

/// `Bind` lets you chain computations together.
///
/// It takes a function `Fn(A) -> M<B>` and applies it to the `A` inside `M<A>`.
/// You can think of this as a callback function for when the value of `A` is
/// ready to be processed, returning the next computation in the sequence.
///
/// This is the primary component of the dreaded [`Monad`](crate::Monad) trait,
/// but to be a [`Monad`](crate::Monad) a type must also implement
/// [`Applicative`](crate::Applicative), which in turn requires implementations
/// for [`Functor`](crate::Functor), [`Pure`](crate::Pure) and
/// [`Apply`](crate::Apply).
pub trait Bind<'a, A>
where
    A: 'a,
{
    type Target<T>
    where
        T: 'a;

    /// Apply the function `f` to the `A` or `A`s inside the `M<A>`, yielding an `M<B>`.
    fn bind<B, F>(self, f: F) -> Self::Target<B>
    where
        B: 'a,
        F: Fn(A) -> Self::Target<B> + 'a;
}

/// `lift_m1` provides a default implementation for
/// [`Functor::fmap`](crate::Functor::fmap) using only [`Bind`](Bind) and
/// [`Pure`](Pure).
pub fn lift_m1<'a, MA, MB, A, B, F>(f: F, a: MA) -> MB
where
    A: 'a,
    B: 'a,
    F: Fn(A) -> B + 'a,
    MA: Bind<'a, A, Target<B> = MB>,
    MB: Pure<B>,
{
    a.bind::<B, _>(move |x| MB::pure(f(x)))
}

impl<'a, A: 'a> Bind<'a, A> for Option<A> {
    type Target<T> = Option<T> where T: 'a;

    fn bind<B, F>(self, f: F) -> Self::Target<B>
    where
        B: 'a,
        F: Fn(A) -> Self::Target<B>,
    {
        self.and_then(f)
    }
}

impl<'a, A: 'a, E> Bind<'a, A> for Result<A, E> {
    type Target<T> = Result<T, E> where T: 'a;

    fn bind<B, F>(self, f: F) -> Self::Target<B>
    where
        B: 'a,
        F: Fn(A) -> Self::Target<B>,
    {
        self.and_then(f)
    }
}

macro_rules! impl_bind_from_iter {
    () => {
        fn bind<B, F>(self, f: F) -> Self::Target<B>
        where
            B: 'a,
            F: Fn(A) -> Self::Target<B>,
        {
            self.into_iter().flat_map(|v| f(v).into_iter()).collect()
        }
    };
}

#[cfg(feature = "std")]
impl<'a, A: 'a> Bind<'a, A> for Vec<A> {
    type Target<T> = Vec<T> where T: 'a;
    impl_bind_from_iter!();
}

#[cfg(feature = "std")]
impl<'a, A: 'a> Bind<'a, A> for std::collections::VecDeque<A> {
    type Target<T> = std::collections::VecDeque<T> where T: 'a;
    impl_bind_from_iter!();
}

#[cfg(feature = "std")]
impl<'a, A: 'a> Bind<'a, A> for std::collections::LinkedList<A> {
    type Target<T> = std::collections::LinkedList<T> where T: 'a;
    impl_bind_from_iter!();
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
