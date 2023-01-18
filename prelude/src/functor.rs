/// A `Functor` lets you change the type parameter of a generic type.
///
/// A `Functor` defines a method `fmap` on a type `F<_>: Functor` which converts
/// an `F<A>` to `F<B>` using a function `Fn(A) -> B` applied to the `A`s inside
/// it.
///
/// You can also use this just to modify the values inside your container value
/// without changing their type, if the mapping function returns a value of the
/// same type.  This is called an "endofunctor."
pub trait Functor<'a, A> {
    type Target<T>;
    fn fmap<B, F>(self, f: F) -> Self::Target<B>
    where
        F: Fn(A) -> B + 'a;
}

impl<A> Functor<'_, A> for Option<A> {
    type Target<T> = Option<T>;

    fn fmap<B, F>(self, f: F) -> Self::Target<B>
    where
        F: Fn(A) -> B,
    {
        self.map(f)
    }
}

impl<A, E> Functor<'_, A> for Result<A, E> {
    type Target<T> = Result<T, E>;

    fn fmap<B, F>(self, f: F) -> Self::Target<B>
    where
        F: Fn(A) -> B,
    {
        self.map(f)
    }
}

#[cfg(feature = "std")]
impl<A> Functor<'_, A> for Vec<A> {
    type Target<T> = Vec<T>;

    fn fmap<B, F>(self, f: F) -> Self::Target<B>
    where
        F: Fn(A) -> B,
    {
        self.into_iter().map(f).collect()
    }
}

#[cfg(feature = "std")]
impl<A> Functor<'_, A> for std::collections::VecDeque<A> {
    type Target<T> = std::collections::VecDeque<T>;

    fn fmap<B, F>(self, f: F) -> Self::Target<B>
    where
        F: Fn(A) -> B,
    {
        self.into_iter().map(f).collect()
    }
}

#[cfg(feature = "std")]
impl<A> Functor<'_, A> for std::collections::LinkedList<A> {
    type Target<T> = std::collections::LinkedList<T>;

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

    #[test]
    fn vec_endofunctor() {
        let a = vec![1, 2, 3, 4, 5];
        let b = a.fmap(|x| x * 2);
        assert_eq!(b, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn vec_exofunctor() {
        let a = vec![1, 2, 3];
        let b = a.fmap(|x| x.to_string());
        assert_eq!(b, vec!["1".to_string(), "2".to_string(), "3".to_string()]);
    }
}
