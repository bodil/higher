/// A `Functor` lets you change the type parameter of a generic type.
///
/// A `Functor` defines a method `fmap` on a type `F<_>: Functor` which converts
/// an `F<A>` to `F<B>` using a function `Fn(A) -> B` applied to the `A`s inside
/// it.
///
/// You can also use this just to modify the values inside your container value
/// without changing their type, if the mapping function returns a value of the
/// same type.  This is called an "endofunctor."
pub trait Functor<A> {
    type Target<T>;
    fn fmap<B, F>(self, f: F) -> Self::Target<B>
    where
        F: Fn(A) -> B;
}

impl<A> Functor<A> for Option<A> {
    type Target<T> = Option<T>;

    fn fmap<B, F>(self, f: F) -> Self::Target<B>
    where
        F: Fn(A) -> B,
    {
        self.map(f)
    }
}

impl<A, E> Functor<A> for Result<A, E> {
    type Target<T> = Result<T, E>;

    fn fmap<B, F>(self, f: F) -> Self::Target<B>
    where
        F: Fn(A) -> B,
    {
        self.map(f)
    }
}

impl<A> Functor<A> for Vec<A> {
    type Target<T> = Vec<T>;

    fn fmap<B, F>(self, f: F) -> Self::Target<B>
    where
        F: Fn(A) -> B,
    {
        self.into_iter().map(f).collect()
    }
}

#[cfg(test)]
mod test {
    use crate::Functor;

    #[test]
    fn option_functor() {
        let a = Option::Some(31337);
        let b = a.fmap(|x| format!("{}", x));
        assert_eq!(b, Option::Some("31337".to_string()));
    }
}
