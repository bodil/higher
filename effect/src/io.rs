use std::convert::identity;
use std::fmt::Display;
use std::fs::File;
use std::future::IntoFuture;
use std::io::{stdin, stdout, Error, Read, Write};
use std::path::Path;

use futures::future::{self, Either};
use futures::{future::LocalBoxFuture, Future, FutureExt};

use higher::ApplicativeError;
use higher::{apply::ApplyFn, Apply, Bifunctor, Bind, Functor, Pure};

/// An IO monad.
///
/// This wraps a [`Future`](Future) similarly to the [`Effect`](crate::Effect)
/// monad, but the wrapped effect must return a [`Result`](Result). Just as with
/// [`Result`](Result)'s [`Bind`](Bind) implementation, this also short circuits
/// a chain of computations if a step produces an error, resolving immediately
/// to that error.
///
/// You can construct an IO monad from a [`Future`](Future) which returns a
/// [`Result`](Result):
///
/// ```
/// # use higher_effect::IO;
/// let my_io_monad = IO::<&str, &str>::from(async { Ok("Hello Joe!") });
/// # assert_eq!(my_io_monad.run(), Ok("Hello Joe!"));
/// ```
///
/// You can `.await` an IO monad and get a [`Result`](Result) back:
///
/// ```
/// # use higher_effect::IO;
/// # use futures::executor::LocalPool;
/// # let my_io_monad = IO::<&str, &str>::from(async { Ok("Hello Joe!") });
/// # LocalPool::new().run_until(async {
/// assert_eq!(my_io_monad.await, Ok("Hello Joe!"));
/// # });
/// ```
///
/// You can run your IO monads on a thread local executor using the
/// [`run()`](IO::run) method. Naturally, we also provide a version of Haskell's
/// [`putStrLn`](https://hackage.haskell.org/package/base-4.17.0.0/docs/Prelude.html#v:putStrLn)
/// so that we can implement the canonical hello world in monadic Rust:
///
/// ```
/// # use higher_effect::io::put_str_ln;
/// put_str_ln("Hello Simon!").run();
/// ```
///
/// Because IO implements [`Bind`](higher::Bind), you can chain async IO
/// operations together using the [`run!`](higher::run) macro:
///
/// ```no_run
/// # use higher::run;
/// # use higher_effect::io::{put_str_ln, put_str, get_line};
/// run! {
///     put_str("What is your name? ");
///     name <= get_line();
///     put_str_ln(format!("Hello {}!", name))
/// }.run();
/// ```
pub struct IO<'a, A, E> {
    state: IOState<'a, A, E>,
}

enum IOState<'a, A, E> {
    Future(LocalBoxFuture<'a, Result<A, E>>),
    Error(E),
}

impl<'a, A, E> IO<'a, A, E> {
    /// Run the effect to completion, blocking the current thread.
    pub fn run(self) -> Result<A, E> {
        futures::executor::LocalPool::new().run_until(self.into_future())
    }

    /// Transform the effect's outputs using the given functions.
    ///
    /// This is identical to [`Bifunctor::bimap`](Bifunctor::bimap), except it
    /// accepts a [`FnOnce`](FnOnce) instead of a [`Fn`](Fn).
    fn map<C, D, L, R>(self, left: L, right: R) -> IO<'a, C, D>
    where
        A: 'a,
        C: 'a,
        D: 'a,
        E: 'a,
        L: FnOnce(A) -> C + 'a,
        R: FnOnce(E) -> D + 'a,
    {
        match self.state {
            IOState::Error(error) => IO::throw_error(right(error)),
            IOState::Future(future) => async move {
                match future.await {
                    Err(error) => Err(right(error)),
                    Ok(result) => Ok(left(result)),
                }
            }
            .into(),
        }
    }

    /// Transform the effect's success value using the given function.
    ///
    /// If the effect fails, the function is not applied.
    ///
    /// This is identical to [`Functor::fmap`](Functor::fmap), except it accepts
    /// a [`FnOnce`](FnOnce) instead of a [`Fn`](Fn).
    pub fn map_ok<B, F>(self, f: F) -> IO<'a, B, E>
    where
        A: 'a,
        B: 'a,
        E: 'a,
        F: FnOnce(A) -> B + 'a,
    {
        self.map(f, identity)
    }

    /// Transform the effect's error value using the given function.
    ///
    /// If the effect succeeds, the function is not applied.
    ///
    /// This is identical to [`Bifunctor::rmap`](Bifunctor::rmap), except it accepts
    /// a [`FnOnce`](FnOnce) instead of a [`Fn`](Fn).
    pub fn map_error<B, F>(self, f: F) -> IO<'a, A, B>
    where
        A: 'a,
        B: 'a,
        E: 'a,
        F: FnOnce(E) -> B + 'a,
    {
        self.map(identity, f)
    }

    /// Test whether this effect is known to have already failed.
    pub fn is_err(&self) -> bool {
        matches!(self.state, IOState::Error(_))
    }

    /// Run two effects in parallel, returning the result of whichever finishes
    /// first, or the error from whichever fails first.
    pub fn try_select<B, EB>(
        eff1: IO<'a, A, E>,
        eff2: IO<'a, B, EB>,
    ) -> IO<
        'a,
        Either<(A, IO<'a, B, EB>), (B, IO<'a, A, E>)>,
        Either<(E, IO<'a, B, EB>), (EB, IO<'a, A, E>)>,
    >
    where
        A: 'a,
        B: 'a,
        E: 'a,
        EB: 'a,
    {
        async {
            futures::future::try_select(eff1.into_future(), eff2.into_future())
                .await
                .bimap(
                    |left| left.bimap(|(a, f)| (a, f.into()), |(b, f)| (b, f.into())),
                    |right| right.bimap(|(a, f)| (a, f.into()), |(b, f)| (b, f.into())),
                )
        }
        .into()
    }

    /// Run a list of effects in parallel, returning the result of whichever
    /// finishes first, or the first error to occur.
    pub fn select_ok<I>(iter: I) -> IO<'a, (A, Vec<IO<'a, A, E>>), E>
    where
        A: 'a,
        E: 'a,
        I: IntoIterator<Item = IO<'a, A, E>> + 'a,
    {
        async {
            futures::future::select_ok(iter.into_iter().map(|eff| eff.into_future()))
                .await
                .map(|(result, remainder)| (result, remainder.fmap(|f| f.into())))
        }
        .into()
    }

    /// Run two effects in parallel, returning the results of both, or the error
    /// from whichever fails first.
    pub fn try_join<B>(eff1: IO<'a, A, E>, eff2: IO<'a, B, E>) -> IO<'a, (A, B), E>
    where
        A: 'a,
        B: 'a,
        E: 'a,
    {
        async { futures::future::try_join(eff1.into_future(), eff2.into_future()).await }.into()
    }

    /// Run three effects in parallel, returning the results of all three, or the error
    /// from whichever fails first.
    pub fn try_join3<B, C>(
        eff1: IO<'a, A, E>,
        eff2: IO<'a, B, E>,
        eff3: IO<'a, C, E>,
    ) -> IO<'a, (A, B, C), E>
    where
        A: 'a,
        B: 'a,
        C: 'a,
        E: 'a,
    {
        async {
            futures::future::try_join3(eff1.into_future(), eff2.into_future(), eff3.into_future())
                .await
        }
        .into()
    }

    /// Run four effects in parallel, returning the results of all four, or the error
    /// from whichever fails first.
    pub fn try_join4<B, C, D>(
        eff1: IO<'a, A, E>,
        eff2: IO<'a, B, E>,
        eff3: IO<'a, C, E>,
        eff4: IO<'a, D, E>,
    ) -> IO<'a, (A, B, C, D), E>
    where
        A: 'a,
        B: 'a,
        C: 'a,
        D: 'a,
        E: 'a,
    {
        async {
            futures::future::try_join4(
                eff1.into_future(),
                eff2.into_future(),
                eff3.into_future(),
                eff4.into_future(),
            )
            .await
        }
        .into()
    }

    /// Run a list of effects in parallel, returning a list of their results, or the error
    /// from whichever fails first.
    pub fn try_join_all<I>(iter: I) -> IO<'a, Vec<A>, E>
    where
        I: IntoIterator<Item = IO<'a, A, E>> + 'a,
    {
        async { futures::future::try_join_all(iter.into_iter().map(|eff| eff.into_future())).await }
            .into()
    }
}

impl<'a, A, E> std::fmt::Debug for IO<'a, A, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
        match self.state {
            IOState::Future(future) => future,
            IOState::Error(error) => Box::pin(async { Err(error) }),
        }
    }
}

impl<'a, A, E, F> From<F> for IO<'a, A, E>
where
    F: Future<Output = Result<A, E>> + 'a,
{
    fn from(future: F) -> Self {
        Self {
            state: IOState::Future(future.boxed_local()),
        }
    }
}

impl<'a, A: 'a, E: 'a> Bind<'a, A> for IO<'a, A, E> {
    type Target<T> = IO<'a, T, E> where T: 'a;

    fn bind<B, F>(self, f: F) -> Self::Target<B>
    where
        B: 'a,
        F: Fn(A) -> Self::Target<B> + 'a,
    {
        match self.state {
            IOState::Error(error) => IO::throw_error(error),
            IOState::Future(future) => async move {
                match future.await {
                    Ok(result) => f(result),
                    Err(error) => IO::throw_error(error),
                }
                .await
            }
            .into(),
        }
    }
}

impl<'a, A: 'a, E: 'a> Functor<'a, A> for IO<'a, A, E> {
    type Target<T> = IO<'a, T, E> where T: 'a;

    fn fmap<B, F>(self, f: F) -> Self::Target<B>
    where
        B: 'a,
        F: Fn(A) -> B + 'a,
    {
        self.map_ok(f)
    }
}

impl<'a, A: 'a, E: 'a> Bifunctor<'a, A, E> for IO<'a, A, E> {
    type Target<T, U> = IO<'a, T, U> where T: 'a, U: 'a;

    fn bimap<A2, E2, L, R>(self, left: L, right: R) -> Self::Target<A2, E2>
    where
        A2: 'a,
        E2: 'a,
        L: Fn(A) -> A2 + 'a,
        R: Fn(E) -> E2 + 'a,
    {
        self.map(left, right)
    }
}

impl<'a, A: 'a, E> Pure<A> for IO<'a, A, E> {
    fn pure(value: A) -> Self {
        async { Ok(value) }.into()
    }
}

impl<'a, A: 'a, E: 'a> Apply<'a, A> for IO<'a, A, E> {
    type Target<T> = IO<'a, T, E> where T: 'a;

    fn apply<B>(
        self,
        f: <Self as Apply<'a, A>>::Target<ApplyFn<'a, A, B>>,
    ) -> <Self as Apply<'a, A>>::Target<B>
    where
        B: 'a,
    {
        async move {
            match future::join(f.into_future(), self.into_future()).await {
                (Err(error), _) => Err(error),
                (_, Err(error)) => Err(error),
                (Ok(func), Ok(arg)) => Ok(func.apply_fn(arg)),
            }
        }
        .into()
    }
}

impl<'a, A: 'a, E: 'a> ApplicativeError<'a, A, E> for IO<'a, A, E> {
    fn throw_error(error: E) -> Self {
        Self {
            state: IOState::Error(error),
        }
    }

    fn handle_error_with<F>(self, f: F) -> Self
    where
        F: Fn(E) -> Self + 'a,
    {
        match self.state {
            IOState::Error(error) => f(error),
            IOState::Future(future) => async move {
                match future.await {
                    Ok(result) => Self::pure(result),
                    Err(error) => f(error),
                }
                .await
            }
            .into(),
        }
    }
}

/// Print a string to the console.
pub fn put_str<'a, S: AsRef<str> + 'a>(s: S) -> IO<'a, (), Error> {
    async move { stdout().write(s.as_ref().as_bytes()).map(|_| ()) }.into()
}

/// Print a string and a newline to the console.
pub fn put_str_ln<'a, S: AsRef<str> + 'a>(s: S) -> IO<'a, (), Error> {
    put_str(format!("{}\n", s.as_ref()))
}

/// Print any value which implements [`Display`](Display) to the console,
/// followed by a newline.
pub fn print<'a, S: Display + 'a>(s: S) -> IO<'a, (), Error> {
    put_str_ln(format!("{}", s))
}

/// Read a line from the console.
pub fn get_line<'a>() -> IO<'a, String, Error> {
    async move {
        let mut s = String::new();
        stdin().read_line(&mut s).map(|_| s)
    }
    .into()
}

/// Read the entire contents of the standard input.
pub fn get_contents<'a>() -> IO<'a, String, Error> {
    async move {
        let mut s = String::new();
        stdin().read_to_string(&mut s).map(|_| s)
    }
    .into()
}

/// Read a file from the filesystem.
pub fn read_file<'a, P: 'a + AsRef<Path>>(path: P) -> IO<'a, Vec<u8>, Error> {
    async move { std::fs::read(path) }.into()
}

/// Write a file to the filesystem.
///
/// If the file already exists, it will be overwritten.
pub fn write_file<'a, P: 'a + AsRef<Path>, C: 'a + AsRef<[u8]>>(
    path: P,
    contents: C,
) -> IO<'a, (), Error> {
    async move { std::fs::write(path, contents) }.into()
}

/// Append to an already existing file on the filesystem.
///
/// If the file does not already exist, it will be created.
pub fn append_file<'a, P: 'a + AsRef<Path>, C: 'a + AsRef<[u8]>>(
    path: P,
    contents: C,
) -> IO<'a, (), Error> {
    async move {
        let mut f = File::options().append(true).open(path.as_ref())?;
        f.write_all(contents.as_ref())
    }
    .into()
}
