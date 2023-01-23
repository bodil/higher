use crate::{Applicative, Bind};

/// A `Monad` is like a burrito, and also anything which implements
/// [`Bind`](Bind) and [`Applicative`](Applicative).
///
/// A monad's primary function is to provide the [`Bind`](Bind) trait, but to
/// count as a monad a type must also implement [`Applicative`](Applicative),
/// which in turn requires you to implement [`Functor`](crate::Functor),
/// [`Pure`](crate::Pure) and [`Apply`](crate::Apply).
pub trait Monad<'a, A: 'a>: Bind<'a, A> + Applicative<'a, A>
where
    A: Clone,
{
}

impl<'a, M, A: 'a> Monad<'a, A> for M
where
    A: Clone,
    M: Bind<'a, A> + Applicative<'a, A>,
{
}

#[cfg(test)]
mod test {
    use crate::{Bind, Pure};

    #[test]
    fn warm_fuzzy_option() {
        let a = Option::pure(31337i32);
        let b = a.bind(|x| Option::pure(x as usize));
        assert_eq!(b, Option::Some(31337usize));
    }

    #[test]
    fn warm_fuzzy_vec() {
        let a = vec![1, 2, 3];
        let b = a.bind(|x| vec![x, x + 1]);
        assert_eq!(b, vec![1, 2, 2, 3, 3, 4]);
    }
}
