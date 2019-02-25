pub trait Lift<A, B> {
    type Target1;
}

pub trait Lift3<A, B, C>: Lift<A, C> {
    type Target2;
    fn cast(
        mut from: <<Self as Lift3<A, B, C>>::Target2 as Lift<B, C>>::Target1,
    ) -> <Self as Lift<A, C>>::Target1
    where
        <Self as Lift3<A, B, C>>::Target2: Lift<B, C>,
    {
        unsafe {
            let ptr = &mut from as *mut _ as *mut <Self as Lift<A, C>>::Target1;
            ::std::ptr::read(ptr)
        }
    }
}

impl<A, B> Lift<A, B> for Option<A> {
    type Target1 = Option<B>;
}

impl<A, B, E> Lift<A, B> for Result<A, E> {
    type Target1 = Result<B, E>;
}

impl<A, B> Lift<A, B> for Vec<A> {
    type Target1 = Vec<B>;
}

impl<A, B, C> Lift3<A, B, C> for Option<A> {
    type Target2 = Option<B>;
}

impl<A, B, C, E> Lift3<A, B, C> for Result<A, E> {
    type Target2 = Result<B, E>;
}

impl<A, B, C> Lift3<A, B, C> for Vec<A> {
    type Target2 = Vec<B>;
}

/// A `Functor` defines a method `map` on a type `F<_>: Functor` which converts
/// an `F<A>` to `F<B>` using a function `Fn(A) -> B`.
pub trait Functor<A, B>: Lift<A, B> {
    fn map<F>(self, f: F) -> <Self as Lift<A, B>>::Target1
    where
        F: Fn(A) -> B;
}

impl<A, B> Functor<A, B> for Option<A> {
    fn map<F>(self, f: F) -> <Self as Lift<A, B>>::Target1
    where
        F: Fn(A) -> B,
    {
        self.map(f)
    }
}

impl<A, B, E> Functor<A, B> for Result<A, E> {
    fn map<F>(self, f: F) -> <Self as Lift<A, B>>::Target1
    where
        F: Fn(A) -> B,
    {
        self.map(f)
    }
}

impl<A, B> Functor<A, B> for Vec<A> {
    fn map<F>(self, f: F) -> <Self as Lift<A, B>>::Target1
    where
        F: Fn(A) -> B,
    {
        self.into_iter().map(f).collect()
    }
}

pub trait Apply<A, B, C>: Functor<A, C> + Lift3<A, B, C>
where
    B: Fn(A) -> C,
{
    fn apply(self, f: <Self as Lift3<A, B, C>>::Target2) -> <Self as Lift<A, C>>::Target1;
}

impl<A, B, C> Apply<A, B, C> for Option<A>
where
    B: Fn(A) -> C,
{
    fn apply(self, f: <Self as Lift3<A, B, C>>::Target2) -> <Self as Lift<A, C>>::Target1 {
        self.and_then(|v| f.map(|f| f(v)))
    }
}

impl<A, B, C, E> Apply<A, B, C> for Result<A, E>
where
    B: Fn(A) -> C,
{
    fn apply(self, f: <Self as Lift3<A, B, C>>::Target2) -> <Self as Lift<A, C>>::Target1 {
        self.and_then(|v| f.map(|f| f(v)))
    }
}

impl<A, B, C> Apply<A, B, C> for Vec<A>
where
    A: Clone,
    B: Fn(A) -> C + Clone,
{
    fn apply(self, f: <Self as Lift3<A, B, C>>::Target2) -> <Self as Lift<A, C>>::Target1 {
        self.bind(|v: A| f.clone().map(|f2| f2(v.clone())))
    }
}

pub trait Pure<A> {
    fn pure(value: A) -> Self;
}

impl<A> Pure<A> for Option<A> {
    fn pure(value: A) -> Self {
        Some(value)
    }
}

impl<A, E> Pure<A> for Result<A, E> {
    fn pure(value: A) -> Self {
        Ok(value)
    }
}

impl<A> Pure<A> for Vec<A> {
    fn pure(value: A) -> Self {
        vec![value]
    }
}

pub trait Applicative<A, F, B>: Functor<A, B> + Apply<A, F, B> + Pure<A>
where
    F: Fn(A) -> B,
{
}

impl<M, A, F, B> Applicative<A, F, B> for M
where
    M: Functor<A, B> + Apply<A, F, B> + Pure<A>,
    F: Fn(A) -> B,
{
}

/// A `Monad` is just the categorical dual of a `Comonad`.
pub trait Bind<A, B>: Lift<A, B> {
    fn bind<F>(self, f: F) -> <Self as Lift<A, B>>::Target1
    where
        F: Fn(A) -> <Self as Lift<A, B>>::Target1;
}

pub trait LiftM1<A, B>: Bind<A, B> {
    fn lift_m1<F>(self, f: F) -> <Self as Lift<A, B>>::Target1
    where
        F: Fn(A) -> B,
        <Self as Lift<A, B>>::Target1: Pure<B>;
}

impl<M, A, B> LiftM1<A, B> for M
where
    M: Bind<A, B>,
{
    fn lift_m1<F>(self, f: F) -> <Self as Lift<A, B>>::Target1
    where
        F: Fn(A) -> B,
        <Self as Lift<A, B>>::Target1: Pure<B>,
    {
        self.bind(|value| Pure::pure(f(value)))
    }
}

pub trait Ap<A, F, B>: Lift3<A, F, B> {
    fn ap(self, f: <Self as Lift3<A, F, B>>::Target2) -> <Self as Lift<A, B>>::Target1
    where
        A: Clone,
        Self: Bind<A, B>,
        <Self as Lift3<A, F, B>>::Target2: Bind<F, B> + Clone,
        <<Self as Lift3<A, F, B>>::Target2 as Lift<F, B>>::Target1: Pure<B>,
        F: Fn(A) -> B;
}

impl<M, A, F, B> Ap<A, F, B> for M
where
    M: Lift3<A, F, B>,
{
    fn ap(self, f: <Self as Lift3<A, F, B>>::Target2) -> <Self as Lift<A, B>>::Target1
    where
        A: Clone,
        Self: Bind<A, B>,
        <Self as Lift3<A, F, B>>::Target2: Bind<F, B> + Clone,
        <<Self as Lift3<A, F, B>>::Target2 as Lift<F, B>>::Target1: Pure<B>,
        F: Fn(A) -> B,
    {
        self.bind(|v: A| {
            let m: <<Self as Lift3<A, F, B>>::Target2 as Lift<F, B>>::Target1 =
                f.clone().bind(|fun: F| Pure::pure(fun(v.clone())));
            Self::cast(m)
        })
    }
}

/// A `Monad` is just the categorical dual of a `Comonad`.
pub trait Monad<A, F, B>: Bind<A, B> + Applicative<A, F, B>
where
    F: Fn(A) -> B,
{
}

impl<M, A, F, B> Monad<A, F, B> for M
where
    M: Bind<A, B> + Applicative<A, F, B>,
    F: Fn(A) -> B,
{
}

impl<A, B> Bind<A, B> for Option<A> {
    fn bind<F>(self, f: F) -> <Self as Lift<A, B>>::Target1
    where
        F: Fn(A) -> <Self as Lift<A, B>>::Target1,
    {
        self.and_then(f)
    }
}

impl<A, B, E> Bind<A, B> for Result<A, E> {
    fn bind<F>(self, f: F) -> <Self as Lift<A, B>>::Target1
    where
        F: Fn(A) -> <Self as Lift<A, B>>::Target1,
    {
        self.and_then(f)
    }
}

impl<A, B> Bind<A, B> for Vec<A> {
    fn bind<F>(self, f: F) -> <Self as Lift<A, B>>::Target1
    where
        F: Fn(A) -> <Self as Lift<A, B>>::Target1,
    {
        self.into_iter().flat_map(|v| f(v).into_iter()).collect()
    }
}

pub trait Bilift<A, B, C, D> {
    type Target;
}

impl<A, B, C, D> Bilift<A, B, C, D> for Result<A, B> {
    type Target = Result<C, D>;
}

/// A `Functor` over two arguments.
pub trait Bifunctor<A, B, C, D>: Bilift<A, B, C, D> {
    fn bimap<L, R>(self, left: L, right: R) -> <Self as Bilift<A, B, C, D>>::Target
    where
        L: Fn(A) -> C,
        R: Fn(B) -> D;
}

pub trait BifunctorLeft<A, B, C>: Bifunctor<A, B, C, B> {
    fn lmap<F>(self, f: F) -> <Self as Bilift<A, B, C, B>>::Target
    where
        F: Fn(A) -> C;
}

impl<A, B, C> BifunctorLeft<A, B, C> for A
where
    A: Bifunctor<A, B, C, B>,
{
    fn lmap<F>(self, f: F) -> <Self as Bilift<A, B, C, B>>::Target
    where
        F: Fn(A) -> C,
    {
        self.bimap(f, |a| a)
    }
}

pub trait BifunctorRight<A, B, C>: Bifunctor<A, B, A, C> {
    fn rmap<F>(self, f: F) -> <Self as Bilift<A, B, A, C>>::Target
    where
        F: Fn(B) -> C;
}

impl<A, B, C> BifunctorRight<A, B, C> for A
where
    A: Bifunctor<A, B, A, C>,
{
    fn rmap<F>(self, f: F) -> <Self as Bilift<A, B, A, C>>::Target
    where
        F: Fn(B) -> C,
    {
        self.bimap(|a| a, f)
    }
}

impl<A, B, C, D> Bifunctor<A, B, C, D> for Result<A, B> {
    fn bimap<L, R>(self, left: L, right: R) -> <Self as Bilift<A, B, C, D>>::Target
    where
        L: Fn(A) -> C,
        R: Fn(B) -> D,
    {
        match self {
            Ok(a) => Ok(left(a)),
            Err(b) => Err(right(b)),
        }
    }
}

/// A `Profunctor` is just a `Bifunctor` that is contravariant over its first
/// argument and covariant over its second argument.
pub trait Profunctor<A, B, C, D>: Bilift<A, B, C, D> + Bifunctor<C, B, A, D> {
    fn dimap<L, R>(self, left: L, right: R) -> <Self as Bilift<A, B, C, D>>::Target
    where
        L: Fn(C) -> A,
        R: Fn(B) -> D;
}

pub trait ProfunctorLeft<A, B, C>: Profunctor<A, B, C, B> {
    fn lcmap<F>(self, f: F) -> <Self as Bilift<A, B, C, B>>::Target
    where
        F: Fn(C) -> A;
}

impl<A, B, C> ProfunctorLeft<A, B, C> for A
where
    A: Profunctor<A, B, C, B>,
{
    fn lcmap<F>(self, f: F) -> <Self as Bilift<A, B, C, B>>::Target
    where
        F: Fn(C) -> A,
    {
        self.dimap(f, |a| a)
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
