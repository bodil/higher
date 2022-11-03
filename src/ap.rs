use crate::{Bind, Pure};

pub fn ap<A, B, F, MA, MB, MF>(f: MF, a: MA) -> MB
where
    F: Fn(A) -> B,
    MA: Bind<A, Target<B> = MB> + Clone,
    MB: Pure<B>,
    MF: Bind<F, Target<B> = MB>,
{
    f.bind::<B, _>(|fv| a.clone().bind::<B, _>(|av| MB::pure(fv(av))))
}
