use crate::{Bind, Pure};

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
