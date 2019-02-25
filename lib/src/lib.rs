pub trait Lift<A, B> {
    type Target;
}

pub trait Lift3<A, B, C> {
    type Target2;
    type Target3;
}

pub trait Lift4<A, B, C, D> {
    type Target2;
    type Target3;
    type Target4;
}

pub trait Lift5<A, B, C, D, E> {
    type Target2;
    type Target3;
    type Target4;
    type Target5;
}

impl<A, B> Lift<A, B> for Option<A> {
    type Target = Option<B>;
}

impl<A, B, E> Lift<A, B> for Result<A, E> {
    type Target = Result<B, E>;
}

impl<A, B> Lift<A, B> for Vec<A> {
    type Target = Vec<B>;
}

impl<A, B, C> Lift3<A, B, C> for Option<A> {
    type Target2 = Option<B>;
    type Target3 = Option<C>;
}

impl<A, B, C, E> Lift3<A, B, C> for Result<A, E> {
    type Target2 = Result<B, E>;
    type Target3 = Result<C, E>;
}

impl<A, B, C> Lift3<A, B, C> for Vec<A> {
    type Target2 = Vec<B>;
    type Target3 = Vec<C>;
}

impl<A, B, C, D> Lift4<A, B, C, D> for Option<A> {
    type Target2 = Option<B>;
    type Target3 = Option<C>;
    type Target4 = Option<D>;
}

impl<A, B, C, D, E> Lift4<A, B, C, D> for Result<A, E> {
    type Target2 = Result<B, E>;
    type Target3 = Result<C, E>;
    type Target4 = Result<D, E>;
}

impl<A, B, C, D> Lift4<A, B, C, D> for Vec<A> {
    type Target2 = Vec<B>;
    type Target3 = Vec<C>;
    type Target4 = Vec<D>;
}

impl<A, B, C, D, E> Lift5<A, B, C, D, E> for Option<A> {
    type Target2 = Option<B>;
    type Target3 = Option<C>;
    type Target4 = Option<D>;
    type Target5 = Option<E>;
}

impl<A, B, C, D, E, Err> Lift5<A, B, C, D, E> for Result<A, Err> {
    type Target2 = Result<B, Err>;
    type Target3 = Result<C, Err>;
    type Target4 = Result<D, Err>;
    type Target5 = Result<D, Err>;
}

impl<A, B, C, D, E> Lift5<A, B, C, D, E> for Vec<A> {
    type Target2 = Vec<B>;
    type Target3 = Vec<C>;
    type Target4 = Vec<D>;
    type Target5 = Vec<E>;
}

pub trait Functor<A, B>: Lift<A, B> {
    fn map<F>(self, f: F) -> <Self as Lift<A, B>>::Target
    where
        F: Fn(A) -> B;
}

impl<A, B> Functor<A, B> for Option<A> {
    fn map<F>(self, f: F) -> <Self as Lift<A, B>>::Target
    where
        F: Fn(A) -> B,
    {
        self.map(f)
    }
}

impl<A, B, E> Functor<A, B> for Result<A, E> {
    fn map<F>(self, f: F) -> <Self as Lift<A, B>>::Target
    where
        F: Fn(A) -> B,
    {
        self.map(f)
    }
}

impl<A, B> Functor<A, B> for Vec<A> {
    fn map<F>(self, f: F) -> <Self as Lift<A, B>>::Target
    where
        F: Fn(A) -> B,
    {
        self.into_iter().map(f).collect()
    }
}

pub trait Applicative<A, B, C>: Functor<A, C> + Lift3<A, B, C>
where
    B: Fn(A) -> C,
{
    fn ap(self, f: <Self as Lift3<A, B, C>>::Target2) -> <Self as Lift3<A, B, C>>::Target3;
}

impl<A, B, C> Applicative<A, B, C> for Option<A>
where
    B: Fn(A) -> C,
{
    fn ap(self, f: <Self as Lift3<A, B, C>>::Target2) -> <Self as Lift3<A, B, C>>::Target3 {
        self.and_then(|v| f.map(|f| f(v)))
    }
}

impl<A, B, C, E> Applicative<A, B, C> for Result<A, E>
where
    B: Fn(A) -> C,
{
    fn ap(self, f: <Self as Lift3<A, B, C>>::Target2) -> <Self as Lift3<A, B, C>>::Target3 {
        self.and_then(|v| f.map(|f| f(v)))
    }
}

impl<A, B, C> Applicative<A, B, C> for Vec<A>
where
    A: Clone,
    B: Fn(A) -> C + Clone,
{
    fn ap(self, f: <Self as Lift3<A, B, C>>::Target2) -> <Self as Lift3<A, B, C>>::Target3 {
        self.bind(|v| f.clone().map(|f2| f2(v.clone())))
    }
}

pub trait Monad<A, B>: Functor<A, B> {
    fn bind<F>(self, f: F) -> <Self as Lift<A, B>>::Target
    where
        F: Fn(A) -> <Self as Lift<A, B>>::Target;
}

impl<A, B> Monad<A, B> for Option<A> {
    fn bind<F>(self, f: F) -> <Self as Lift<A, B>>::Target
    where
        F: Fn(A) -> <Self as Lift<A, B>>::Target,
    {
        self.and_then(f)
    }
}

impl<A, B, E> Monad<A, B> for Result<A, E> {
    fn bind<F>(self, f: F) -> <Self as Lift<A, B>>::Target
    where
        F: Fn(A) -> <Self as Lift<A, B>>::Target,
    {
        self.and_then(f)
    }
}

impl<A, B> Monad<A, B> for Vec<A> {
    fn bind<F>(self, f: F) -> <Self as Lift<A, B>>::Target
    where
        F: Fn(A) -> <Self as Lift<A, B>>::Target,
    {
        self.into_iter().flat_map(|v| f(v).into_iter()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flatmap_that_shit() {
        assert_eq!(
            Some("1337".to_string()),
            Some(1337).bind(|n| Some(format!("{}", n)))
        );
        assert_eq!(vec![1, 1, 2, 2, 3, 3], vec![1, 2, 3].bind(|v| vec![v, v]))
    }
}
