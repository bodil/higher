#![deny(unsafe_code, nonstandard_style)]
#![forbid(rust_2018_idioms)]
#![warn(unreachable_pub, missing_debug_implementations)]
#![cfg_attr(not(feature = "std"), no_std)]

mod semigroup;
pub use crate::semigroup::Semigroup;

mod monoid;
pub use crate::monoid::Monoid;

mod functor;
pub use crate::functor::Functor;

mod contra;
pub use crate::contra::Contravariant;

mod bifunctor;
pub use crate::bifunctor::Bifunctor;

mod profunctor;
pub use crate::profunctor::Profunctor;

mod pure;
pub use crate::pure::Pure;

mod apply;
pub use crate::apply::Apply;

mod bind;
pub use crate::bind::Bind;

mod applicative;
pub use crate::applicative::Applicative;

mod monad;
pub use crate::monad::Monad;

mod extract;
pub use crate::extract::Extract;

mod extend;
pub use crate::extend::Extend;

mod comonad;
pub use crate::comonad::Comonad;

mod ap;
pub use crate::ap::ap;

mod liftm1;
pub use crate::liftm1::lift_m1;

pub mod foldable;
pub use crate::foldable::Foldable;
