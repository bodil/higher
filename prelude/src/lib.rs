#![deny(unsafe_code, nonstandard_style)]
#![forbid(rust_2018_idioms)]
#![warn(unreachable_pub, missing_debug_implementations)]
#![allow(rustdoc::bare_urls)]
#![doc = include_str!("../README.md")]

pub use higher_derive::{Bifunctor, BifunctorRef, Functor, FunctorRef};

pub mod semigroup;
#[doc(inline)]
pub use crate::semigroup::Semigroup;

pub mod monoid;
#[doc(inline)]
pub use crate::monoid::Monoid;

pub mod functor;
#[doc(inline)]
pub use crate::functor::{Functor, FunctorRef};

pub mod contra;
#[doc(inline)]
pub use crate::contra::Contravariant;

pub mod bifunctor;
#[doc(inline)]
pub use crate::bifunctor::{Bifunctor, BifunctorRef};

pub mod profunctor;
#[doc(inline)]
pub use crate::profunctor::Profunctor;

pub mod pure;
#[doc(inline)]
pub use crate::pure::Pure;

pub mod apply;
#[doc(inline)]
pub use crate::apply::Apply;

pub mod bind;
#[doc(inline)]
pub use crate::bind::Bind;

pub mod applicative;
#[doc(inline)]
pub use crate::applicative::Applicative;

pub mod monad;
#[doc(inline)]
pub use crate::monad::Monad;

pub mod foldable;
#[doc(inline)]
pub use crate::foldable::Foldable;

pub mod traversable;
#[doc(inline)]
pub use crate::traversable::Traversable;

pub mod error;
#[doc(inline)]
pub use crate::error::{ApplicativeError, MonadError};

pub mod alt;
#[doc(inline)]
pub use crate::alt::Alt;

pub mod plus;
#[doc(inline)]
pub use crate::plus::Plus;

pub mod alternative;
#[doc(inline)]
pub use crate::alternative::Alternative;

pub mod monadplus;
#[doc(inline)]
pub use crate::monadplus::MonadPlus;

pub mod algebras;
pub mod rings;

/// Monadic do notation.
///
/// This macro provides some syntactic sugar to make monads easier to read and
/// write.
///
/// It takes a series of expressions evaluating to monads, separated by
/// semicolons, and chains them together using [`bind`](Bind::bind).
///
/// The last expression may be preceded by the `yield` keyword, which will
/// automatically wrap the result up into a monad using [`pure`](Pure::pure).
/// Otherwise it must be a plain expression returning a monad.
///
/// Expressions before the last may be binding expressions, binding the result
/// of the monadic computation to a named variable available to the subsequent
/// expressions. If they don't have a binding, the result of the computation is
/// discarded. A binding expression looks like this:
///
/// ```text
/// variable <= expression;
/// ```
///
/// # Examples
///
/// The simplest example of monadic do notation is using the [`Option`](Option)
/// type. It will run through the list of expressions as long as they keep
/// evaluating to [`Some`](Option::Some), but if an expression should return
/// [`None`](Option::None), it will discard the subsequent computations and
/// immediately return [`None`](Option::None).
///
/// We'll illustrate with integer division using
/// [`usize::checked_div`](usize::checked_div), which returns an
/// `Option<usize>`:
///
/// ```
/// # use higher::run;
/// # assert_eq!(
/// run! {
///     x <= 16usize.checked_div(2);
///     // x = 16 / 2 = 8
///     y <= x.checked_div(2);
///     // y = x / 2 = 8 / 2 = 4
///     z <= y.checked_div(2);
///     // z = y / 2 = 4 / 2 = 2
///     yield x + y + z
///     // returns Some(x + y + z) = Some(8 + 4 + 2) = Some(14)
/// }
/// # , Some(14));
/// ```
///
/// And for a failing example, when we divide by zero:
///
/// ```
/// # use higher::run;
/// # assert_eq!(
/// run! {
///     x <= 16usize.checked_div(2);
///     // x = 16 / 2 = 8
///     y <= x.checked_div(0);
///     // Division by zero returns None, the remaining expressions are ignored
///     z <= y.checked_div(2);
///     yield x + y + z
///     // returns None
/// }
/// # , None);
/// ```

#[macro_export]
macro_rules! run {
    //matching against a token tree, because a pattern cannot be followed by "<=".
    //To still get good error messages, a nested macro is used, that matches against pat.
    ($binding:tt <= <$coerce:ty> $comp:expr; $($tail:tt)*) => { run!{$binding <= [] <$coerce> $comp; $($tail)*} };

    ($binding:tt <= [$($shadow_clone:ident),*] <$coerce:ty> $comp:expr; $($tail:tt)*) => {
        {
            macro_rules! verify_pat { ($_:pat_param) => {}; } verify_pat!($binding);
            $(let $shadow_clone = $shadow_clone.clone();)*
            $crate::Bind::bind::<$coerce, _>($comp, move |$binding| run!($($tail)*))
        }
    };

    ($binding:tt <= $comp:expr; $($tail:tt)*) => { run!{$binding <= [] $comp; $($tail)*} };

    ($binding:tt <= [$($shadow_clone:ident),*] $comp:expr; $($tail:tt)*) => {
        {
            macro_rules! verify_pat { ($_:pat_param) => {}; } verify_pat!($binding);
            $(let $shadow_clone = $shadow_clone.clone();)*
            $crate::Bind::bind($comp, move |$binding| run!($($tail)*))
        }
    };

    (<$coerce:ty> $comp:expr; $($tail:tt)*) => { run!{[] <$coerce> $comp; $($tail)*} };

    ([$($shadow_clone:ident),*] <$coerce:ty> $comp:expr; $($tail:tt)*) => {
        {
            $(let $shadow_clone = $shadow_clone.clone();)*
            $crate::Bind::bind::<$coerce, _>($comp, move |_| run!($($tail)*))
        }
    };

    ($comp:expr; $($tail:tt)*) => {run!{[] $comp; $($tail)*}};

    ([$($shadow_clone:ident),*] $comp:expr; $($tail:tt)*) => {
        {
            $(let $shadow_clone = $shadow_clone.clone();)*
            $crate::Bind::bind($comp, move |_| run!($($tail)*))
        }
    };

    (yield $result:expr) => { run!{[] yield $result} };

    ([$($shadow_clone:ident),*] yield $result:expr) => {
        {
            $(let $shadow_clone = $shadow_clone.clone();)*
            $crate::Pure::pure($result)
        }
    };

    ($result:expr) => { run!{[] $result} };

    ([$($shadow_clone:ident),*] $result:expr) => {
        {
            $(let $shadow_clone = $shadow_clone.clone();)*
            $result
        }
    };
}


/// Construct a function that ignores its argument and returns the same value
/// every time you call it.
///
/// You may know this function as `const` in certain other languages.
///
/// ```
/// # use higher::repeat;
/// let f = repeat(31337);
/// assert_eq!(f("Joe"), 31337);
/// assert_eq!(f("Mike"), 31337);
/// assert_eq!(f("Robert"), 31337);
/// assert_eq!(f("Bjarne"), 31337);
/// ```
pub fn repeat<A: Clone, B>(value: A) -> impl Fn(B) -> A {
    move |_| value.clone()
}

#[cfg(test)]
mod test {
    #[test]
    fn do_notation() {
        // The standard list monad test.
        assert_eq!(
            run! {
                x <= vec![1, 2];
                y <= vec![x, x + 1];
                yield (x, y)
            },
            vec![(1, 1), (1, 2), (2, 2), (2, 3)]
        );

        // Option with yield.
        assert_eq!(
            run! {
                x <= 25u32.checked_div(2);
                y <= x.checked_div(3);
                z <= 9u32.checked_div(y);
                yield x + y + z
            },
            Some(18)
        );

        // Option which fails.
        assert_eq!(
            run! {
                x <= 5u32.checked_div(2);
                y <= x.checked_div(8);
                z <= 9u32.checked_div(y);
                yield x + y + z
            },
            None
        );

        // Option with manual wrap.
        assert_eq!(
            run! {
                x <= 25u32.checked_div(2);
                y <= x.checked_div(3);
                z <= 9u32.checked_div(y);
                Some(x + y + z)
            },
            Some(18)
        );

        // Option with tuple destructure.
        assert_eq!(
            run! {
                (a,b) <= Some((3u32, 32.0f32));
                (a,_b) <= Some((b as u32, a));
                yield a
            },
            Some(32u32)
        );

        // Option without binding.
        assert_eq!(
            run! {
                2u32.checked_div(2);
                2u32.checked_div(1)
            },
            Some(2)
        );

        // Option without binding which fails.
        assert_eq!(
            run! {
                2u32.checked_div(0);
                2u32.checked_div(1)
            },
            None
        );

        // Options with different types.
        assert_eq!(
            run! {
                s <= Some("64");
                x <= s.parse::<u32>().ok();
                y <= x.checked_div(2);
                yield y
            },
            Some(32)
        );

        // Explicit clone in do-notation
        #[derive(Clone,PartialEq, Debug)] struct NoCopy<A>(A);
        assert_eq!(
            run!{
                t <= run! {
                    t <= Some(NoCopy(5u32)); //tests the syntax of assign without explicit clone.
                    Some(3i32); //here ownership of t is still clear, no clone needed. Tests syntax of non-assigning without clone.
                    [t] Some(4i32); //here t would need multiple owners already. Explicit clone.
                    v <= [t] <NoCopy<u32>> Some(t); //test for the syntax with coerce, clone and assign.
                    g <= <NoCopy<u32>> Some(v.clone()); //test for the syntax with coerce and assign.
                    h <= [v] Some(NoCopy(v.0+1)); // tests the syntax of assign with explicit clone.
                    [g] Some(g); //test the syntax of not-assigning with clone.
                    [h] yield h //yield with explicit clone.
                };
                t <= run!{Some(t)}; //tests the syntax of returning an expression without explicit clone.
                t <= run!{
                    t <= Some(t);
                    Some(4);
                    [t] Some(t) //tests the syntax of returning an expression with explicit clone.
                };
                run!{yield t} //yield without explicit clone.
            },
            Some(NoCopy(6u32))
        );

    }
}
