use std::{
    collections::{LinkedList, VecDeque},
    rc::Rc,
};

use crate::{functor::FunctorRef, repeat, run, Bind, Functor, Pure};

/// An `ApplyFn` is a function from `A` to `B` wrapped in something Rust's
/// type system can more easily digest. Arguments for
/// [`Apply::apply()`](Apply::apply) are required to be of this type rather than
/// an arbitrary type matching `Fn(A) -> B`.
///
/// Create an `ApplyFn` by using its [`From`](From) implementation:
///
/// ```
/// # use higher::apply::ApplyFn;
/// let f = ApplyFn::from(|x| x + 2);
/// assert_eq!(f.apply_fn(3), 5);
/// ```
pub struct ApplyFn<'a, A, B> {
    function: Rc<dyn Fn(A) -> B + 'a>,
}

impl<'a, A, B> ApplyFn<'a, A, B> {
    /// Apply the wrapped function to a value of type `A`.
    pub fn apply_fn(&self, a: A) -> B {
        (self.function)(a)
    }
}

impl<'a, A, B> Clone for ApplyFn<'a, A, B> {
    fn clone(&self) -> Self {
        Self {
            function: self.function.clone(),
        }
    }
}

impl<'a, A, B, F> From<F> for ApplyFn<'a, A, B>
where
    F: 'a + Fn(A) -> B,
{
    fn from(f: F) -> Self {
        ApplyFn {
            function: Rc::new(f),
        }
    }
}

impl<'a, A, B> std::fmt::Debug for ApplyFn<'a, A, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "ApplyFn({}) -> {}",
            std::any::type_name::<A>(),
            std::any::type_name::<B>()
        ))
    }
}

/// `Apply` takes an `Apply<Fn(A) -> B>` (or, rather, an `Apply<ApplyFn<'a, A,
/// B>>` specifically) and applies it to an `Apply<A>` to produce an `Apply<B>`.
///
/// In simpler terms, it takes a value of type `A` inside some context ("some
/// context" here being the type which implements `Apply`) and a function `Fn(A)
/// ->B` inside a similar context and produces a similar context containing the
/// result (`B`) of applying the function to the value. Some concrete examples
/// of "some context" are [`Option`](Option), [`Vec`](Vec) and futures.
pub trait Apply<'a, A: 'a>: Functor<'a, A> {
    /// Apply an `F` of functions from `A` to `B` to an `F` of `A`,
    /// producing an `F` of `B`.
    fn apply<B: 'a>(self, f: Self::Target<ApplyFn<'a, A, B>>) -> Self::Target<B>;

    fn apply_first<B: 'a>(self, b: Self::Target<B>) -> Self
    where
        Self: Sized,
        A: Clone,

        Self::Target<B>: Apply<'a, B>
            + Functor<'a, B, Target<ApplyFn<'a, B, A>> = Self::Target<ApplyFn<'a, B, A>>>
            + Functor<'a, B, Target<A> = Self>,
    {
        let mapped: Self::Target<ApplyFn<'a, B, A>> = self.fmap(|x: A| ApplyFn::from(repeat(x)));
        b.apply(mapped)
    }

    fn apply_second<B: 'a>(self, b: Self::Target<B>) -> Self::Target<B>
    where
        Self: Sized,
        B: Clone,

        Self::Target<B>: Apply<'a, B>
            + Functor<'a, B, Target<ApplyFn<'a, A, B>> = Self::Target<ApplyFn<'a, A, B>>>
            + Functor<'a, B, Target<A> = Self>,
    {
        self.apply(b.fmap(|x: B| ApplyFn::from(repeat(x))))
    }
}

/// `ap` is a default implementation of [`Apply::apply`][Apply::apply] for any
/// type that implements [`Bind`](Bind), [`Pure`](Pure) and
/// [`FunctorRef`](FunctorRef), where the contained type `A` implements
/// [`Clone`](Clone).
///
/// This is the easy way to implement [`Apply`](Apply):
///
/// ```
/// # use higher::apply::{Apply, ApplyFn, ap};
/// # use higher::{Bind, Pure, Functor, FunctorRef};
/// #[derive(Clone, Functor, FunctorRef)]
/// enum AustralianOption<A> {
///   NahYeah(A),
///   YeahNah
/// }
///
/// impl<'a, A: 'a> Bind<'a, A> for AustralianOption<A> {
///     fn bind<B: 'a, F>(self, f: F) -> Self::Target<B>
///     where F: Fn(A) -> Self::Target<B> + 'a {
///         match self {
///             Self::NahYeah(x) => f(x),
///             Self::YeahNah => AustralianOption::YeahNah,
///         }
///     }
/// }
///
/// impl<A> Pure<A> for AustralianOption<A> {
///     fn pure(value: A) -> Self {
///         Self::NahYeah(value)
///     }
/// }
///
/// impl<'a, A: Clone + 'a> Apply<'a, A> for AustralianOption<A> {
///     fn apply<B: 'a>(self, f: Self::Target<ApplyFn<'a, A, B>>) -> Self::Target<B> {
///         ap(f, self)
///     }
/// }
/// ```
pub fn ap<'a, A: 'a, B: 'a, M: 'a>(mf: M::Target<ApplyFn<'a, A, B>>, ma: M) -> M::Target<B>
where
    M: Bind<'a, A> + Pure<A> + FunctorRef<'a, A, Target<A> = M> + Clone,

    M::Target<B>: Bind<'a, B> + Pure<B>,
    M::Target<ApplyFn<'a, A, B>>:
        Bind<'a, ApplyFn<'a, A, B>, Target<B> = M::Target<B>> + Pure<ApplyFn<'a, A, B>>,
{
    run! {
        f <= <B> mf;
        a <= <B> ma.clone();
        yield f.apply_fn(a)
    }
}

pub fn lift2<'a, A: 'a, B: 'a, C: 'a, F: 'a, M>(f: &'a F, a: M, b: M::Target<B>) -> M::Target<C>
where
    A: Clone,
    F: Fn(A, B) -> C,
    M: Apply<'a, A>,

    M::Target<B>: Apply<'a, B, Target<C> = M::Target<C>>
        + Apply<'a, B, Target<ApplyFn<'a, B, C>> = M::Target<ApplyFn<'a, B, C>>>,
    M::Target<C>: Apply<'a, C, Target<B> = M::Target<B>>
        + Apply<'a, C, Target<ApplyFn<'a, B, C>> = M::Target<ApplyFn<'a, B, C>>>,
{
    b.apply(a.fmap(move |x: A| ApplyFn::from(move |y: B| f(x.clone(), y))))
}

impl<'a, A: 'a> Apply<'a, A> for Option<A>
where
    A: Clone,
{
    fn apply<B>(self, f: Self::Target<ApplyFn<'a, A, B>>) -> Self::Target<B>
    where
        B: 'a,
    {
        self.and_then(|x| f.map(|f| f.apply_fn(x)))
    }
}

impl<'a, A: 'a, E> Apply<'a, A> for Result<A, E>
where
    A: Clone,
{
    fn apply<B>(self, f: Self::Target<ApplyFn<'a, A, B>>) -> Self::Target<B>
    where
        B: 'a,
    {
        self.and_then(|x| f.map(|f| f.apply_fn(x)))
    }
}

macro_rules! impl_apply_for_list {
    ($type:ident) => {
        impl<'a, A: 'a> Apply<'a, A> for $type<A>
        where
            A: Clone,
        {
            fn apply<B: 'a>(self, f: Self::Target<ApplyFn<'a, A, B>>) -> Self::Target<B> {
                ap(f, self)
            }
        }
    };
}

impl_apply_for_list!(Vec);
impl_apply_for_list!(VecDeque);
impl_apply_for_list!(LinkedList);

#[cfg(test)]
mod test {
    use super::{lift2, Apply, ApplyFn};

    #[test]
    fn apply_option() {
        let n: Option<i32> = None;
        let nf: Option<ApplyFn<'_, i32, i32>> = None;
        assert_eq!(
            Some(5i32).apply(Some::<ApplyFn<'_, i32, i32>>(ApplyFn::from(|x: i32| x + 2))),
            Some(7i32)
        );
        assert_eq!(n.apply::<i32>(Some(ApplyFn::from(|x: i32| x + 2))), None);
        assert_eq!(Some(5i32).apply::<i32>(nf), None);
    }

    #[test]
    fn apply_vec() {
        let a = vec![1, 2, 3];
        let f = vec![
            ApplyFn::from(|x: i32| x + 3),
            ApplyFn::from(|x: i32| x + 2),
            ApplyFn::from(|x: i32| x + 1),
        ];
        assert_eq!(a.apply(f), vec![4, 5, 6, 3, 4, 5, 2, 3, 4]);
    }

    #[test]
    fn apply_lift2() {
        let a = Some(2);
        let b = Some(3);
        let sum = lift2(&|a, b| a + b, a, b);
        assert_eq!(sum, Some(5));
    }
}
