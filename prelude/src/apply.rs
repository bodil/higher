use crate::{Bind, Functor, Pure};

/// An `ApplyFn` is a function from `A` to `B` wrapped in something Rust's type
/// system can more easily digest. Arguments for [Apply::apply()] are required
/// to be of this type rather than an arbitrary type matching `Fn(A) -> B`.
pub struct ApplyFn<A, B> {
    function: Box<dyn Fn(A) -> B>,
}

impl<A, B> ApplyFn<A, B> {
    /// Apply the wrapped function to a value of type `A`.
    pub fn apply(&self, a: A) -> B {
        (self.function)(a)
    }
}

impl<A, B, F> From<F> for ApplyFn<A, B>
where
    F: 'static + Fn(A) -> B,
{
    fn from(f: F) -> Self {
        ApplyFn {
            function: Box::new(f),
        }
    }
}

impl<A, B> core::fmt::Debug for ApplyFn<A, B> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&format!(
            "ApplyFn({}) -> {}",
            std::any::type_name::<A>(),
            std::any::type_name::<B>()
        ))
    }
}

pub fn f<A, B, F>(func: F) -> ApplyFn<A, B>
where
    F: 'static + Fn(A) -> B,
{
    ApplyFn::from(func)
}

/// `Apply` takes an `F<Fn(A) -> B>` (or, rather, an `F<ApplyFn<A, B>>`
/// specifically) and applies it to an `F<A>` to produce an `F<B>`.
pub trait Apply<A>: Functor<A> {
    type Target<T>;
    fn apply<B>(
        self,
        f: <Self as Apply<A>>::Target<ApplyFn<A, B>>,
    ) -> <Self as Apply<A>>::Target<B>;
}

pub fn ap<A, B, MA, MB, MF>(f: MF, a: MA) -> MB
where
    MA: Bind<A, Target<B> = MB> + Clone,
    MB: Pure<B>,
    MF: Bind<ApplyFn<A, B>, Target<B> = MB>,
{
    f.bind::<B, _>(|fv| a.clone().bind::<B, _>(|av| MB::pure(fv.apply(av))))
}

pub fn apply_first<A, B, MA, MB, MF>(a: MA, b: MB) -> MA
where
    A: Clone + 'static,
    MA: Apply<A, Target<B> = MB> + Functor<A, Target<ApplyFn<B, A>> = MF>,
    MB: Apply<B, Target<ApplyFn<B, A>> = MF> + Apply<B, Target<A> = MA>,
    MF: Apply<ApplyFn<B, A>, Target<B> = MB>,
{
    b.apply(a.fmap(|x: A| f(move |_| x.clone())))
}

pub fn apply_second<A, B, MA, MB, MF>(a: MA, b: MB) -> MB
where
    B: Clone + 'static,
    MA: Apply<A, Target<B> = MB> + Apply<A, Target<ApplyFn<A, B>> = MF>,
    MB: Apply<B, Target<ApplyFn<A, B>> = MF> + Functor<B, Target<ApplyFn<A, B>> = MF>,
    MF: Apply<ApplyFn<A, B>, Target<A> = MA>,
{
    a.apply(b.fmap(|x: B| f(move |_| x.clone())))
}

pub fn lift2<A, B, C, MA, MB, MC, MF, F>(fun: F, a: MA, b: MB) -> MC
where
    F: Fn(A, B) -> C + 'static,
    A: Clone + 'static,
    B: Clone,
    MA: Apply<A, Target<C> = MC> + Functor<A, Target<ApplyFn<B, C>> = MF>,
    MB: Apply<B, Target<C> = MC> + Apply<B, Target<ApplyFn<B, C>> = MF>,
    MC: Apply<C, Target<A> = MA>,
    MF: Apply<ApplyFn<B, C>>,
{
    let fun_ref = std::rc::Rc::new(fun);
    b.apply(a.fmap(move |x: A| {
        let fun_int = fun_ref.clone();
        f(move |y: B| fun_int(x.clone(), y))
    }))
}

impl<A> Apply<A> for Option<A> {
    type Target<T> = Option<T>;

    fn apply<B>(
        self,
        f: <Self as Apply<A>>::Target<ApplyFn<A, B>>,
    ) -> <Self as Apply<A>>::Target<B> {
        self.and_then(|x| f.map(|f| f.apply(x)))
    }
}

impl<A, E> Apply<A> for Result<A, E> {
    type Target<T> = Result<T, E>;

    fn apply<B>(
        self,
        f: <Self as Apply<A>>::Target<ApplyFn<A, B>>,
    ) -> <Self as Apply<A>>::Target<B> {
        self.and_then(|x| f.map(|f| f.apply(x)))
    }
}

#[cfg(feature = "std")]
impl<A> Apply<A> for Vec<A>
where
    Vec<A>: Clone,
{
    type Target<T> = Vec<T>;

    fn apply<B>(
        self,
        f: <Self as Apply<A>>::Target<ApplyFn<A, B>>,
    ) -> <Self as Apply<A>>::Target<B> {
        ap(f, self)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        apply::{f, ApplyFn},
        Apply,
    };

    #[test]
    fn apply_option() {
        let n: Option<i32> = None;
        let nf: Option<ApplyFn<i32, i32>> = None;
        assert_eq!(Some(5).apply(Some(f(|x| x + 2))), Some(7));
        assert_eq!(n.apply(Some(f(|x| x + 2))), None);
        assert_eq!(Some(5).apply(nf), None);
    }

    #[cfg(feature = "std")]
    mod std_test {
        use crate::apply::{f, Apply};

        #[test]
        fn apply_vec() {
            let a = vec![1, 2, 3];
            let f = vec![f(|x: i32| x + 3), f(|x: i32| x + 2), f(|x: i32| x + 1)];
            assert_eq!(a.apply(f), vec![4, 5, 6, 3, 4, 5, 2, 3, 4]);
        }
    }
}
