use crate::{run, Bind, Functor, Pure};

pub trait ApplicativeError<'a, A: 'a, E: 'a>: Functor<'a, A> + Pure<A> {
    /// Throw an error.
    ///
    /// This constructs an error case for the
    /// [`ApplicativeError`](ApplicativeError).
    fn throw_error(error: E) -> Self;

    /// Handle an error.
    ///
    /// If the [`ApplicativeError`](ApplicativeError) contains an error, handle
    /// it using the provided function, returning a new
    /// [`ApplicativeError`](ApplicativeError).
    ///
    /// If there's no error, do nothing.
    fn handle_error_with<F: 'a>(self, f: F) -> Self
    where
        F: Fn(E) -> Self;

    /// Handle an error.
    ///
    /// If the [`ApplicativeError`](ApplicativeError) contains an error, handle
    /// it using the provided function, returning a success value which will be
    /// wrapped in a new [`ApplicativeError`](ApplicativeError).
    ///
    /// If there's no error, do nothing.
    fn handle_error<F: 'a>(self, f: F) -> Self
    where
        Self: Sized,
        F: Fn(E) -> A,
    {
        self.handle_error_with(move |e| Pure::pure(f(e)))
    }

    fn attempt(self) -> Self::Target<Result<A, E>>
    where
        Self: Sized,

        Self::Target<Result<A, E>>: ApplicativeError<'a, A, E> + Pure<Result<A, E>>,
    {
        self.fmap(|v| Ok(v))
            .handle_error_with(|error| Pure::pure(Err(error)))
    }

    fn recover_with<F>(self, recover: F) -> Self
    where
        Self: Sized,
        F: Fn(E) -> Result<Self, E> + 'a,
    {
        self.handle_error_with(move |error| recover(error).unwrap_or_else(Self::throw_error))
    }

    fn recover<F>(self, recover: F) -> Self
    where
        Self: Sized,
        F: Fn(E) -> Result<A, E> + 'a,
    {
        self.recover_with(move |error| recover(error).map(Self::pure))
    }

    fn adapt_error<F>(self, adapt: F) -> Self
    where
        Self: Sized,
        F: Fn(E) -> E + 'a,
    {
        self.recover_with(move |error| Err(adapt(error)))
    }

    fn redeem<B: 'a, FE: 'a, FA: 'a>(self, recover: FE, map: FA) -> Self::Target<B>
    where
        Self: Sized,
        FE: Fn(E) -> B,
        FA: Fn(A) -> B,

        Self::Target<B>: ApplicativeError<'a, B, E>,
    {
        self.fmap(map).handle_error(recover)
    }

    fn from_option<F>(option: Option<A>, if_error: F) -> Self
    where
        Self: Sized,
        F: FnOnce() -> E,
    {
        option.map_or_else(|| Self::throw_error(if_error()), Self::pure)
    }

    fn from_result(result: Result<A, E>) -> Self
    where
        Self: Sized,
    {
        result.map_or_else(Self::throw_error, Self::pure)
    }
}

pub trait MonadError<'a, A: 'a, E: 'a>: ApplicativeError<'a, A, E> + Bind<'a, A> {
    fn rethrow(mr: Self::Target<Result<A, E>>) -> Self
    where
        Self: Sized,

        Self::Target<Result<A, E>>: Bind<'a, Result<A, E>, Target<A> = Self>,
    {
        run! {
            result <= <A>mr;
            result.map_or_else(Self::throw_error, Self::pure)
        }
    }

    fn ensure_or<P: 'a, F: 'a>(self, error: F, predicate: P) -> Self
    where
        Self: Sized,
        P: Fn(&A) -> bool,
        F: Fn(&A) -> E,

        Self: Bind<'a, A, Target<A> = Self>,
    {
        run! {
            result <= <A>self;
            if predicate(&result) {
                Self::pure(result)
            } else {
                Self::throw_error(error(&result))
            }
        }
    }

    fn ensure<P: 'a>(self, error: E, predicate: P) -> Self
    where
        Self: Sized,
        E: Clone,
        P: Fn(&A) -> bool,

        Self: Bind<'a, A, Target<A> = Self>,
    {
        self.ensure_or(move |_| error.clone(), predicate)
    }

    fn redeem_with<B: 'a, FA: 'a, FE: 'a>(self, recover: FE, bind: FA) -> Self::Target<B>
    where
        Self: Sized + Bind<'a, A, Target<A> = Self>,
        FE: Fn(E) -> Self::Target<B>,
        FA: Fn(A) -> Self::Target<B>,

        Self::Target<B>: Bind<'a, B, Target<A> = Self>,
        Self::Target<Result<A, E>>: Bind<'a, Result<A, E>, Target<B> = Self::Target<B>>
            + Pure<Result<A, E>>
            + ApplicativeError<'a, A, E>,
    {
        run! {
            result <= <B>self.attempt();
            result.map_or_else(&recover, &bind)
        }
    }
}

impl<'a, A: 'a, E: 'a, M> MonadError<'a, A, E> for M where
    M: ApplicativeError<'a, A, E> + Bind<'a, A>
{
}

impl<'a, A: 'a> ApplicativeError<'a, A, ()> for Option<A> {
    fn throw_error(_error: ()) -> Self::Target<A> {
        None
    }

    fn handle_error_with<F>(self, f: F) -> Self::Target<A>
    where
        F: FnOnce(()) -> Self::Target<A> + 'a,
    {
        self.or_else(|| f(()))
    }
}

impl<'a, A: 'a, E: 'a> ApplicativeError<'a, A, E> for Result<A, E> {
    fn throw_error(error: E) -> Self::Target<A> {
        Err(error)
    }

    fn handle_error_with<F>(self, f: F) -> Self::Target<A>
    where
        F: FnOnce(E) -> Self::Target<A> + 'a,
    {
        self.or_else(f)
    }
}
