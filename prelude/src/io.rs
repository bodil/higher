use std::future::IntoFuture;
use std::io::{stdout, Error, Write};

use futures::executor::LocalPool;
use futures::future::LocalBoxFuture;
use futures::Future;

use crate::{Apply, Bind, Functor, Pure};

/// An IO monad.
///
/// This wraps a [`Future`](Future) similarly to the
/// [`Effect`](crate::effect::Effect) monad, but the wrapped effect must return
/// a [`Result`](Result). Just as with [`Result`](Result)'s [`Bind`](Bind)
/// implementation, this also short circuits a chain of computations if a step
/// produces an error, resolving immediately to that error.
///
/// You can construct an IO monad from a [`Future`](Future) which returns a
/// [`Result`](Result):
///
/// ```
/// # use higher::io::{IO, run_io};
/// let my_io_monad = IO::<&str, &str>::from(async { Ok("Hello Joe!") });
/// # assert_eq!(run_io(my_io_monad), Ok("Hello Joe!"));
/// ```
///
/// You can `.await` an IO monad and get a [`Result`](Result) back:
///
/// ```
/// # use higher::io::{IO, run_io};
/// # use futures::executor::LocalPool;
/// # let my_io_monad = IO::<&str, &str>::from(async { Ok("Hello Joe!") });
/// # LocalPool::new().run_until(async {
/// assert_eq!(my_io_monad.await, Ok("Hello Joe!"));
/// # });
/// ```
///
/// You can run your IO monads on a thread local executor using the
/// [`run_io`](run_io) function. Naturally, we also provide a version of
/// Haskell's
/// [`putStrLn`](https://hackage.haskell.org/package/base-4.17.0.0/docs/Prelude.html#v:putStrLn)
/// so that we can implement the canonical hello world in monadic Rust:
///
/// ```
/// # use higher::io::{run_io, putstrln};
/// run_io(putstrln("Hello Simon!"));
/// ```
///
/// Because IO implements [`Bind`](crate::Bind), you can chain async IO
/// operations together using the [`run!`](crate::run) macro:
///
/// ```
/// # use higher::{run, io::{putstrln, run_io}};
/// run_io(run! {
///     putstrln("I have the power");
///     putstrln("of");
///     putstrln("HASKELL")
/// });
/// ```
pub enum IO<'a, A, E> {
    Future(LocalBoxFuture<'a, Result<A, E>>),
    Error(E),
}

impl<'a, A, E> IO<'a, A, E> {
    pub fn from_error(error: E) -> Self {
        Self::Error(error)
    }

    pub fn map<B, F>(self, f: F) -> IO<'a, B, E>
    where
        A: 'a,
        E: 'a,
        F: FnOnce(A) -> B + 'a,
    {
        match self {
            Self::Error(error) => IO::from_error(error),
            Self::Future(future) => async move {
                match future.await {
                    Err(error) => Err(error),
                    Ok(result) => Ok(f(result)),
                }
            }
            .into(),
        }
    }

    pub fn map_error<B, F>(self, f: F) -> IO<'a, A, B>
    where
        A: 'a,
        E: 'a,
        F: FnOnce(E) -> B + 'a,
    {
        match self {
            Self::Error(error) => IO::from_error(f(error)),
            Self::Future(future) => async {
                match future.await {
                    Err(error) => Err(f(error)),
                    Ok(result) => Ok(result),
                }
            }
            .into(),
        }
    }

    pub fn is_err(&self) -> bool {
        matches!(self, Self::Error(_))
    }
}

impl<'a, A, E> core::fmt::Debug for IO<'a, A, E> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&format!(
            "IO<{}, {}>",
            std::any::type_name::<A>(),
            std::any::type_name::<E>(),
        ))
    }
}

impl<'a, A, E: 'a> IntoFuture for IO<'a, A, E> {
    type Output = Result<A, E>;

    type IntoFuture = LocalBoxFuture<'a, Result<A, E>>;

    fn into_future(self) -> Self::IntoFuture {
        match self {
            Self::Future(future) => future,
            Self::Error(error) => Box::pin(async { Err(error) }),
        }
    }
}

impl<'a, A, E, F> From<F> for IO<'a, A, E>
where
    F: Future<Output = Result<A, E>> + 'a,
{
    fn from(future: F) -> Self {
        Self::Future(Box::pin(future))
    }
}

impl<'a, A: 'a, E: 'a> Bind<'a, A> for IO<'a, A, E> {
    type Target<T> = IO<'a, T, E>;

    fn bind<B, F>(self, f: F) -> Self::Target<B>
    where
        F: Fn(A) -> Self::Target<B> + 'a,
    {
        match self {
            Self::Error(error) => <Self::Target<B>>::from_error(error),
            Self::Future(future) => async move {
                match future.await {
                    Ok(result) => f(result),
                    Err(error) => <Self::Target<B>>::from_error(error),
                }
                .await
            }
            .into(),
        }
    }
}

impl<'a, A: 'a, E: 'a> Functor<'a, A> for IO<'a, A, E> {
    type Target<T> = IO<'a, T, E>;

    fn fmap<B, F>(self, f: F) -> Self::Target<B>
    where
        F: Fn(A) -> B + 'a,
    {
        self.map(f)
    }
}

impl<'a, A: 'a, E> Pure<A> for IO<'a, A, E> {
    fn pure(value: A) -> Self {
        async move { Ok(value) }.into()
    }
}

impl<'a, A: 'a, E: 'a> Apply<'a, A> for IO<'a, A, E> {
    type Target<T> = IO<'a, T, E> where T: 'a;

    fn apply<B>(
        self,
        f: <Self as Apply<'a, A>>::Target<crate::apply::ApplyFn<'a, A, B>>,
    ) -> <Self as Apply<'a, A>>::Target<B>
    where
        B: 'a,
    {
        async move {
            match (f.await, self.await) {
                (Err(error), _) => Err(error),
                (_, Err(error)) => Err(error),
                (Ok(func), Ok(arg)) => Ok(func.apply(arg)),
            }
        }
        .into()
    }
}

pub fn run_io<A, E>(io: IO<'_, A, E>) -> Result<A, E> {
    LocalPool::new().run_until(io.into_future())
}

pub fn putstrln<'a, S: AsRef<str> + 'a>(s: S) -> IO<'a, (), Error> {
    async move {
        stdout()
            .write(format!("{}\n", s.as_ref()).as_bytes())
            .map(|_| ())
    }
    .into()
}
