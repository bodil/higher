#![deny(unsafe_code, nonstandard_style)]
#![forbid(rust_2018_idioms)]
#![warn(unreachable_pub, missing_debug_implementations)]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod semigroup;
pub use crate::semigroup::Semigroup;

pub mod monoid;
pub use crate::monoid::Monoid;

pub mod semiring;
pub use crate::semiring::Semiring;

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
