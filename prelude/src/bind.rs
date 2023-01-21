#[cfg(feature = "futures")]
use futures::{channel::mpsc, stream::LocalBoxStream};

use crate::{run, Pure};

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

    /// Apply the function `f` to the `A` or `A`s inside the `M<A>` to turn it
    /// into an `M<B>`.
    ///
    /// Think of this as a way to chain two computations together: `f` can be
    /// seen as a callback function for a computation: `m.bind(|result| { ... })`
    /// means "run `m` and call this function when the result is ready." The
    /// return value from `f`, of type `M<B>`, is effectively the original
    /// computation, `M<A>`, followed by the `M<B>` from the callback function
    /// when the result of `M<A>` is ready.
    fn bind<B, F>(self, f: F) -> Self::Target<B>
    where
        B: 'a,
        F: Fn(A) -> Self::Target<B> + 'a;

    /// Turn an `M<A>` into an `M<()>` and a
    /// [`Stream<Item = A>`](futures::stream::Stream) that will yield values of
    /// `A`.
    ///
    /// The values won't start yielding until you run the `M<()>`, whatever that
    /// means for any given `M`. For simple types like [`Result`](Result) and
    /// [`Vec`](Vec), you don't have to do anything, they'll yield their entire
    /// contents immediately. For effect types, you'll need to await their
    /// futures to get them to start yielding.
    ///
    /// ```
    /// # use higher::{Bind, Pure};
    /// # use futures::stream::StreamExt;
    /// # futures::executor::LocalPool::new().run_until(async {
    /// let list = vec![1, 2, 3];
    /// let (_, mut stream) = list.into_stream();
    /// assert_eq!(stream.next().await, Some(1));
    /// assert_eq!(stream.next().await, Some(2));
    /// assert_eq!(stream.next().await, Some(3));
    /// assert_eq!(stream.next().await, None);
    /// # });
    /// ```
    #[cfg(feature = "futures")]
    fn into_stream(self) -> (Self::Target<()>, LocalBoxStream<'a, A>)
    where
        Self: Sized + Bind<'a, A>,
        <Self as Bind<'a, A>>::Target<()>: Bind<'a, (), Target<A> = Self> + Pure<()>,
    {
        let (giver, receiver) = mpsc::unbounded();
        let void = self.bind::<(), _>(move |a| Pure::pure(giver.unbounded_send(a).unwrap()));
        (void, Box::pin(receiver))
    }
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
    run! {
        x <= <B> a;
        yield f(x)
    }
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

    #[cfg(feature = "futures")]
    fn into_stream(self) -> (Self::Target<()>, LocalBoxStream<'a, A>)
    where
        Self: Sized + Bind<'a, A>,
        <Self as Bind<'a, A>>::Target<()>: Pure<()>,
    {
        (
            Pure::pure(()),
            match self {
                None => Box::pin(futures::stream::empty()),
                Some(a) => Box::pin(futures::stream::once(async { a })),
            },
        )
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

    #[cfg(feature = "futures")]
    fn into_stream(self) -> (Self::Target<()>, LocalBoxStream<'a, A>)
    where
        Self: Sized + Bind<'a, A>,
        <Self as Bind<'a, A>>::Target<()>: Bind<'a, (), Target<A> = Self> + Pure<()>,
    {
        match self {
            Err(err) => (
                Err(err).bind(|_| Pure::pure(())),
                Box::pin(futures::stream::empty()),
            ),
            Ok(a) => (Pure::pure(()), Box::pin(futures::stream::once(async { a }))),
        }
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

impl<'a, A: 'a> Bind<'a, A> for Vec<A> {
    type Target<T> = Vec<T> where T: 'a;
    impl_bind_from_iter!();
}

impl<'a, A: 'a> Bind<'a, A> for std::collections::VecDeque<A> {
    type Target<T> = std::collections::VecDeque<T> where T: 'a;
    impl_bind_from_iter!();
}

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
