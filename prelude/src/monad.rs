use crate::{Applicative, Bind};

/// A `Monad` is like a burrito, and also anything which implements
/// [`Bind`](Bind) and [`Applicative`](Applicative).
pub trait Monad<A>: Bind<A> + Applicative<A> {}

impl<M, A> Monad<A> for M where M: Bind<A> + Applicative<A> {}

#[cfg(test)]
mod test {
    use crate::{Bind, Pure};

    #[test]
    fn warm_fuzzy_option() {
        let a = Option::pure(31337);
        let b = a.bind(|x| Option::pure(x.to_string()));
        assert_eq!(b, Option::Some("31337".to_string()));
    }

    #[test]
    fn warm_fuzzy_vec() {
        let a = vec![1, 2, 3];
        let b = a.bind(|x| vec![x, x + 1]);
        assert_eq!(b, vec![1, 2, 2, 3, 3, 4]);
    }
}
