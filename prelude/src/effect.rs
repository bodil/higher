use std::future::IntoFuture;

use futures::{executor::LocalPool, future::LocalBoxFuture, Future};

use crate::{Apply, Bind, Functor, Pure};

/// An effect monad.
///
/// This is essentially just a newtype for a boxed [`Future`](Future) that
/// implements [`Monad`](crate::Monad), so that you can treat your
/// [`Future`](Future)s like they're [`Monad`](crate::Monad)s with the
/// [`run!`](crate::run) macro and all the category theory you like.
///
/// To turn a [`Future`](Future) into an [`Effect`](Effect) monad, use
/// [`From`](From) and [`Into`](Into):
///
/// ```
/// # use higher::effect::Effect;
/// let my_future = async { "Hello Joe!" };
/// let my_effect = Effect::from(my_future);
/// ```
///
/// Effects can be awaited like they're futures, because they implement
/// [`IntoFuture`](IntoFuture):
///
/// ```
/// # use higher::effect::Effect;
/// # let mut pool = futures::executor::LocalPool::new();
/// # let my_effect = Effect::from(async { "Hello Joe!" });
/// # let async_block = async {
/// assert_eq!(my_effect.await, "Hello Joe!");
/// # };
/// # pool.run_until(async_block);
/// ```
///
/// You can compose effects using the [`run!`](crate::run) macro:
///
/// ```
/// # use higher::{run, Pure, effect::Effect};
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

impl<'a, A> core::fmt::Debug for Effect<'a, A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
            future: Box::pin(future),
        }
    }
}

impl<'a, A> Bind<'a, A> for Effect<'a, A>
where
    A: 'a,
{
    type Target<T> = Effect<'a, T>;

    fn bind<B, F>(self, f: F) -> Self::Target<B>
    where
        F: Fn(A) -> Self::Target<B> + 'a,
    {
        async move { f(self.await).await }.into()
    }
}

impl<'a, A> Functor<'a, A> for Effect<'a, A>
where
    A: 'a,
{
    type Target<T> = Effect<'a, T>;

    fn fmap<B, F>(self, f: F) -> Self::Target<B>
    where
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
        async move { value }.into()
    }
}

impl<'a, A> Apply<'a, A> for Effect<'a, A>
where
    A: 'a,
{
    type Target<T> = Effect<'a, T> where T:'a;

    fn apply<B>(
        self,
        f: <Self as Apply<'a, A>>::Target<crate::apply::ApplyFn<'a, A, B>>,
    ) -> <Self as Apply<'a, A>>::Target<B>
    where
        B: 'a,
    {
        async move {
            let func = f.await;
            let arg = self.await;
            func.apply(arg)
        }
        .into()
    }
}

pub fn run_effect<A>(effect: Effect<'_, A>) -> A {
    LocalPool::new().run_until(effect.into_future())
}
