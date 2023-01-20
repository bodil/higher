use crate::{run, Bind, Functor, Pure};

/// An `ApplyFn` is a function from `A` to `B` wrapped in something Rust's type
/// system can more easily digest. Arguments for
/// [`Apply::apply()`](Apply::apply) are required to be of this type rather than
/// an arbitrary type matching `Fn(A) -> B`.
pub struct ApplyFn<'a, A, B> {
    function: Box<dyn Fn(A) -> B + 'a>,
}

impl<'a, A, B> ApplyFn<'a, A, B> {
    /// Apply the wrapped function to a value of type `A`.
    pub fn apply(&self, a: A) -> B {
        (self.function)(a)
    }
}

impl<'a, A, B, F> From<F> for ApplyFn<'a, A, B>
where
    F: 'a + Fn(A) -> B,
{
    fn from(f: F) -> Self {
        ApplyFn {
            function: Box::new(f),
        }
    }
}

impl<'a, A, B> core::fmt::Debug for ApplyFn<'a, A, B> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&format!(
            "ApplyFn({}) -> {}",
            std::any::type_name::<A>(),
            std::any::type_name::<B>()
        ))
    }
}

/// `Apply` takes an `F<Fn(A) -> B>` (or, rather, an `F<ApplyFn<'a,A, B>>`
/// specifically) and applies it to an `F<A>` to produce an `F<B>`.
pub trait Apply<'a, A>
where
    A: 'a,
{
    type Target<T>
    where
        T: 'a,
        A: 'a;

    /// Apply an `F` of functions from `A` to `B` to an `F` of `A`,
    /// producing an `F` of `B`.
    fn apply<B>(self, f: Self::Target<ApplyFn<'a, A, B>>) -> Self::Target<B>
    where
        B: 'a;
}

/// `ap` is a default implementation of [`Apply::apply`][Apply::apply] for any
/// type that implements [`Bind`](Bind), [`Pure`](Pure) and [`Clone`](Clone).
///
/// This is the easy way to implement [`Apply`](Apply):
///
/// ```
/// # use higher::apply::{Apply, ApplyFn, ap};
/// # use higher::{Bind, Pure};
/// #[derive(Clone)]
/// enum AustralianOption<A> {
///   NahYeah(A),
///   YeahNah
/// }
///
/// impl<'a, A: 'a> Bind<'a, A> for AustralianOption<A> {
///     type Target<T: 'a> = AustralianOption<T>;
///
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
/// impl<'a, A: 'a> Apply<'a, A> for AustralianOption<A>
/// where
///     AustralianOption<A>: Clone
/// {
///     type Target<T: 'a> = AustralianOption<T>;
///
///     fn apply<B>(self, f: Self::Target<ApplyFn<'a, A, B>>) -> Self::Target<B>
///     where
///         B: 'a,
///     {
///         ap(f, self)
///     }
/// }
/// ```
pub fn ap<'a, A, B, MA, MB, MF>(mf: MF, ma: MA) -> MB
where
    A: 'a,
    B: 'a,
    MA: Bind<'a, A, Target<B> = MB> + Clone + 'a,
    MB: Pure<B>,
    MF: Bind<'a, ApplyFn<'a, A, B>, Target<B> = MB>,
{
    run! {
        f <= <B> mf;
        a <= <B> ma.clone();
        yield f.apply(a)
    }
}

pub fn apply_first<'a, A, B, MA, MB, MF>(a: MA, b: MB) -> MA
where
    A: Clone + 'static,
    B: 'a,
    MA: Apply<'a, A, Target<B> = MB> + Functor<'a, A, Target<ApplyFn<'a, B, A>> = MF>,
    MB: Apply<'a, B, Target<ApplyFn<'a, B, A>> = MF> + Apply<'a, B, Target<A> = MA>,
    MF: Apply<'a, ApplyFn<'a, B, A>, Target<B> = MB>,
{
    b.apply(a.fmap(|x: A| ApplyFn::from(move |_| x.clone())))
}

pub fn apply_second<'a, A, B, MA, MB, MF>(a: MA, b: MB) -> MB
where
    A: 'a,
    B: Clone + 'static,
    MA: Apply<'a, A, Target<B> = MB> + Apply<'a, A, Target<ApplyFn<'a, A, B>> = MF>,
    MB: Apply<'a, B, Target<ApplyFn<'a, A, B>> = MF>
        + Functor<'a, B, Target<ApplyFn<'a, A, B>> = MF>,
    MF: Apply<'a, ApplyFn<'a, A, B>, Target<A> = MA>,
{
    a.apply(b.fmap(|x: B| ApplyFn::from(move |_| x.clone())))
}

pub fn lift2<'a, A, B, C, MA, MB, MC, MF, F>(fun: F, a: MA, b: MB) -> MC
where
    F: Fn(A, B) -> C + 'static,
    A: Clone + 'static,
    B: Clone + 'a,
    C: 'a,
    MA: Apply<'a, A, Target<C> = MC> + Functor<'a, A, Target<ApplyFn<'a, B, C>> = MF>,
    MB: Apply<'a, B, Target<C> = MC> + Apply<'a, B, Target<ApplyFn<'a, B, C>> = MF>,
    MC: Apply<'a, C, Target<A> = MA>,
    MF: Apply<'a, ApplyFn<'a, B, C>>,
{
    let fun_ref = std::rc::Rc::new(fun);
    b.apply(a.fmap(move |x: A| {
        let fun_int = fun_ref.clone();
        ApplyFn::from(move |y: B| fun_int(x.clone(), y))
    }))
}

impl<'a, A> Apply<'a, A> for Option<A>
where
    A: 'a,
{
    type Target<T> = Option<T> where T:'a;

    fn apply<B>(self, f: Self::Target<ApplyFn<'a, A, B>>) -> Self::Target<B>
    where
        B: 'a,
    {
        self.and_then(|x| f.map(|f| f.apply(x)))
    }
}

impl<'a, A, E> Apply<'a, A> for Result<A, E>
where
    A: 'a,
{
    type Target<T> = Result<T, E> where T:'a;

    fn apply<B>(self, f: Self::Target<ApplyFn<'a, A, B>>) -> Self::Target<B>
    where
        B: 'a,
    {
        self.and_then(|x| f.map(|f| f.apply(x)))
    }
}

#[cfg(feature = "std")]
impl<'a, A> Apply<'a, A> for Vec<A>
where
    A: 'a,
    Vec<A>: Clone,
{
    type Target<T> = Vec<T> where T: 'a;

    fn apply<B>(self, f: Self::Target<ApplyFn<'a, A, B>>) -> Self::Target<B>
    where
        B: 'a,
    {
        ap(f, self)
    }
}

#[cfg(feature = "std")]
impl<'a, A> Apply<'a, A> for std::collections::VecDeque<A>
where
    A: 'a,
    std::collections::VecDeque<A>: Clone,
{
    type Target<T> = std::collections::VecDeque<T> where T:'a;

    fn apply<B>(self, f: Self::Target<ApplyFn<'a, A, B>>) -> Self::Target<B>
    where
        B: 'a,
    {
        ap(f, self)
    }
}

#[cfg(feature = "std")]
impl<'a, A> Apply<'a, A> for std::collections::LinkedList<A>
where
    A: 'a,
    std::collections::LinkedList<A>: Clone,
{
    type Target<T> = std::collections::LinkedList<T> where T:'a;

    fn apply<B>(self, f: Self::Target<ApplyFn<'a, A, B>>) -> Self::Target<B>
    where
        B: 'a,
    {
        ap(f, self)
    }
}

#[cfg(test)]
mod test {
    use crate::{apply::ApplyFn, Apply};

    #[test]
    fn apply_option() {
        let n: Option<i32> = None;
        let nf: Option<ApplyFn<'_, i32, i32>> = None;
        assert_eq!(Some(5).apply(Some(ApplyFn::from(|x| x + 2))), Some(7));
        assert_eq!(n.apply(Some(ApplyFn::from(|x| x + 2))), None);
        assert_eq!(Some(5).apply(nf), None);
    }

    #[cfg(feature = "std")]
    mod std_test {
        use crate::apply::{Apply, ApplyFn};

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
    }
}
