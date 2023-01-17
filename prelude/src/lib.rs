#![deny(unsafe_code, nonstandard_style)]
#![forbid(rust_2018_idioms)]
#![warn(unreachable_pub, missing_debug_implementations)]
#![cfg_attr(not(feature = "std"), no_std)]

pub use higher_derive::{Bifunctor, Functor};

pub mod semigroup;
pub use crate::semigroup::Semigroup;

pub mod monoid;
pub use crate::monoid::Monoid;

pub mod semiring;
pub use crate::semiring::Semiring;

pub mod ring;
pub use crate::ring::Ring;

pub mod commutative_ring;
pub use crate::commutative_ring::CommutativeRing;

pub mod euclidean_ring;
pub use crate::euclidean_ring::EuclideanRing;

pub mod division_ring;
pub use crate::division_ring::DivisionRing;

pub mod field;
pub use crate::field::Field;

pub mod heyting_algebra;
pub use crate::heyting_algebra::HeytingAlgebra;

pub mod boolean_algebra;
pub use crate::boolean_algebra::BooleanAlgebra;

pub mod functor;
pub use crate::functor::Functor;

pub mod contra;
pub use crate::contra::Contravariant;

pub mod bifunctor;
pub use crate::bifunctor::Bifunctor;

pub mod profunctor;
pub use crate::profunctor::Profunctor;

pub mod pure;
pub use crate::pure::Pure;

pub mod apply;
pub use crate::apply::Apply;

pub mod bind;
pub use crate::bind::Bind;

pub mod applicative;
pub use crate::applicative::Applicative;

pub mod monad;
pub use crate::monad::Monad;

pub mod extract;
pub use crate::extract::Extract;

pub mod extend;
pub use crate::extend::Extend;

pub mod comonad;
pub use crate::comonad::Comonad;

pub mod foldable;
pub use crate::foldable::Foldable;

#[macro_export]
macro_rules! run {
    (yield $result:expr) => {
        $crate::Pure::pure($result)
    };

    ($result:expr) => {
        $result
    };

    ($binding:ident <= $comp:expr; $($tail:tt)*) => {
        $crate::Bind::bind($comp, |$binding| run!($($tail)*))
    };

    ($comp:expr; $($tail:tt)*) => {
        $crate::Bind::bind($comp, |_| run!($($tail)*))
    }
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
    }
}
