#![deny(unsafe_code, nonstandard_style)]
#![forbid(rust_2018_idioms)]
#![warn(unreachable_pub, missing_debug_implementations)]
#![allow(rustdoc::bare_urls)]
#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

pub use higher_derive::{Bifunctor, Functor};

pub mod semigroup;
#[doc(inline)]
pub use crate::semigroup::Semigroup;

pub mod monoid;
#[doc(inline)]
pub use crate::monoid::Monoid;

pub mod functor;
#[doc(inline)]
pub use crate::functor::Functor;

pub mod contra;
#[doc(inline)]
pub use crate::contra::Contravariant;

pub mod bifunctor;
#[doc(inline)]
pub use crate::bifunctor::Bifunctor;

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

pub mod error;
#[doc(inline)]
pub use crate::error::ApplicativeError;

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
    ($binding:ident <= <$coerce:ident> $comp:expr; $($tail:tt)*) => {
        $crate::Bind::bind::<$coerce, _>($comp, move |$binding| run!($($tail)*))
    };

    ($binding:ident <= $comp:expr; $($tail:tt)*) => {
        $crate::Bind::bind($comp, move |$binding| run!($($tail)*))
    };

    (<$coerce:ident> $comp:expr; $($tail:tt)*) => {
        $crate::Bind::bind::<$coerce, _>($comp, move |_| run!($($tail)*))
    };

    ($comp:expr; $($tail:tt)*) => {
        $crate::Bind::bind($comp, move |_| run!($($tail)*))
    };

    (yield $result:expr) => {
        $crate::Pure::pure($result)
    };

    ($result:expr) => {
        $result
    };

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
    }
}
