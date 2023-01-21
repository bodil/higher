use std::{fmt::Debug, future::IntoFuture};

use futures::{
    future::{Either, LocalBoxFuture},
    Future, FutureExt,
};

use higher::{apply::ApplyFn, Apply, Bifunctor, Bind, Functor, Pure};

/// An effect monad.
///
/// This is essentially just a newtype for a boxed [`Future`](Future) that
/// implements [`Monad`](higher::Monad), so that you can treat your
/// [`Future`](Future)s like they're [`Monad`](higher::Monad)s with the
/// [`run!`](higher::run) macro and all the category theory you like.
///
/// To turn a [`Future`](Future) into an [`Effect`](Effect) monad, use
/// [`From`](From) and [`Into`](Into):
///
/// ```
/// # use higher_effect::Effect;
/// let my_future = async { "Hello Joe!" };
/// let my_effect = Effect::from(my_future);
/// ```
///
/// Effects can be awaited like they're futures, because they implement
/// [`IntoFuture`](IntoFuture):
///
/// ```
/// # use higher_effect::Effect;
/// # let mut pool = futures::executor::LocalPool::new();
/// # let my_effect = Effect::from(async { "Hello Joe!" });
/// # let async_block = async {
/// assert_eq!(my_effect.await, "Hello Joe!");
/// # };
/// # pool.run_until(async_block);
/// ```
///
/// You can compose effects using the [`run!`](higher::run) macro:
///
/// ```
/// # use higher::{run, Pure};
/// # use higher_effect::Effect;
/// # let mut pool = futures::executor::LocalPool::new();
/// # let async_block = async {
/// run! {
///     // Lift the value 1 into the Effect monad
///     x <= Effect::pure(1);
///     // Create an effect from an async block returning the value 2
///     y <= Effect::from(async { 2 });
///     // Perform a computation in an async block using the previously bound values
///     z <= Effect::from(async move { x + y });
///     // Compute the result and await it
///     yield x + y + z
/// }.await
/// # };
/// # assert_eq!(pool.run_until(async_block), 6);
/// ```
pub struct Effect<'a, A> {
    future: LocalBoxFuture<'a, A>,
}

impl<'a, A> Effect<'a, A> {
    /// Run the effect to completion, blocking the current thread.
    pub fn run(self) -> A {
        futures::executor::LocalPool::new().run_until(self.into_future())
    }

    /// Construct an effect that resolves immediately to the given value.
    pub fn ready(value: A) -> Self
    where
        A: 'a,
    {
        async { value }.into()
    }

    /// Runs two effects in parallel, returning the result of whichever finishes first.
    pub fn select<B>(
        eff1: Effect<'a, A>,
        eff2: Effect<'a, B>,
    ) -> Effect<'a, Either<(A, Effect<'a, B>), (B, Effect<'a, A>)>>
    where
        A: 'a,
        B: 'a,
    {
        async {
            futures::future::select(eff1.into_future(), eff2.into_future())
                .await
                .bimap(|(a, f)| (a, f.into()), |(b, f)| (b, f.into()))
        }
        .into()
    }

    /// Runs a list of effects in parallel, returning the result of whichever finishes first.
    pub fn select_all<I>(iter: I) -> Effect<'a, (A, usize, Vec<Effect<'a, A>>)>
    where
        A: 'a,
        I: IntoIterator<Item = Effect<'a, A>> + 'a,
    {
        async {
            let (result, index, remainder) =
                futures::future::select_all(iter.into_iter().map(|eff| eff.into_future())).await;
            (result, index, remainder.fmap(|f| f.into()))
        }
        .into()
    }

    /// Runs two effects in parallel, returning the results of both.
    pub fn join<B>(eff1: Effect<'a, A>, eff2: Effect<'a, B>) -> Effect<'a, (A, B)>
    where
        A: 'a,
        B: 'a,
    {
        async { futures::future::join(eff1.into_future(), eff2.into_future()).await }.into()
    }

    /// Runs three effects in parallel, returning the results of all three.
    pub fn join3<B, C>(
        eff1: Effect<'a, A>,
        eff2: Effect<'a, B>,
        eff3: Effect<'a, C>,
    ) -> Effect<'a, (A, B, C)>
    where
        A: 'a,
        B: 'a,
        C: 'a,
    {
        async {
            futures::future::join3(eff1.into_future(), eff2.into_future(), eff3.into_future()).await
        }
        .into()
    }

    /// Runs four effects in parallel, returning the results of all four.
    pub fn join4<B, C, D>(
        eff1: Effect<'a, A>,
        eff2: Effect<'a, B>,
        eff3: Effect<'a, C>,
        eff4: Effect<'a, D>,
    ) -> Effect<'a, (A, B, C, D)>
    where
        A: 'a,
        B: 'a,
        C: 'a,
        D: 'a,
    {
        async {
            futures::future::join4(
                eff1.into_future(),
                eff2.into_future(),
                eff3.into_future(),
                eff4.into_future(),
            )
            .await
        }
        .into()
    }

    /// Runs a list of effects in parallel, returning a list of their results
    /// when they have all resolved.
    pub fn join_all<I>(iter: I) -> Effect<'a, Vec<A>>
    where
        I: IntoIterator<Item = Effect<'a, A>> + 'a,
    {
        async { futures::future::join_all(iter.into_iter().map(|eff| eff.into_future())).await }
            .into()
    }
}

impl<'a, A> Debug for Effect<'a, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Effect<{}>", std::any::type_name::<A>()))
    }
}

impl<'a, A> IntoFuture for Effect<'a, A> {
    type Output = A;

    type IntoFuture = LocalBoxFuture<'a, A>;

    fn into_future(self) -> Self::IntoFuture {
        self.future
    }
}

impl<'a, A, F> From<F> for Effect<'a, A>
where
    F: Future<Output = A> + 'a,
{
    fn from(future: F) -> Self {
        Self {
            future: future.boxed_local(),
        }
    }
}

impl<'a, A> Bind<'a, A> for Effect<'a, A>
where
    A: 'a,
{
    type Target<T> = Effect<'a, T> where T: 'a;

    fn bind<B, F>(self, f: F) -> Self::Target<B>
    where
        B: 'a,
        F: Fn(A) -> Self::Target<B> + 'a,
    {
        async move { f(self.await).await }.into()
    }
}

impl<'a, A> Functor<'a, A> for Effect<'a, A>
where
    A: 'a,
{
    type Target<T> = Effect<'a, T> where T: 'a;

    fn fmap<B, F>(self, f: F) -> Self::Target<B>
    where
        B: 'a,
        F: Fn(A) -> B + 'a,
    {
        async move { f(self.await) }.into()
    }
}

impl<'a, A> Pure<A> for Effect<'a, A>
where
    A: 'a,
{
    fn pure(value: A) -> Self {
        Self::ready(value)
    }
}

impl<'a, A> Apply<'a, A> for Effect<'a, A>
where
    A: 'a,
{
    type Target<T> = Effect<'a, T> where T:'a;

    fn apply<B>(
        self,
        f: <Self as Apply<'a, A>>::Target<ApplyFn<'a, A, B>>,
    ) -> <Self as Apply<'a, A>>::Target<B>
    where
        B: 'a,
    {
        async move {
            let (func, arg) = Effect::join(f, self).await;
            func.apply(arg)
        }
        .into()
    }
}
