//! The `Functor` hierarchy using [`higher`][higher].
//!
//! # But Why?
//!
//! This is just a proof of concept implementation of PureScript's functor
//! hierarchy in Rust, and you should probably not use it extensively in your
//! own code, not because it's buggy or unfinished but because it's not likely
//! to produce very nice Rust code. I'm sorry, but this is Rust, it works
//! differently from Haskell.
//!
//! Nevertheless, if you ever wanted comonads and profunctors in Rust, you've
//! got them.
//!
//! # What Is The Functor Hierarchy?
//!
//! Honestly, if you didn't already learn this from Haskell or Scala or
//! PureScript, put down the crate and back away. As mentioned above, you are
//! much better off using Rusty idioms to write your code, and I would recommend
//! learning about these concepts in the context of a language (such as Haskell)
//! where they belong.
//!
//! If you still want to learn about functors and applicatives and monads, I
//! highly recommend the [Category Theory for Programmers][bartosz] series.
//!
//! # Custom Derives
//!
//! The [`higher-derive`][higher-derive] crate provides a custom derive for
//! `Functor`:
//!
//! ```
//! # use higher_derive::{Lift, Functor};
//! # use higher::Lift;
//! # use higher_cat::Functor;
//! # fn main() {
//! #[derive(Lift, Functor, PartialEq, Debug)]
//! enum MyLittleOption<A> {
//!     Definitely(A),
//!     NotReally,
//! }
//!
//! // The derive will map any variant field of type `A`:
//! let i = MyLittleOption::Definitely(123);
//! let o = i.map(|value: u8| value.to_string());
//! assert_eq!(MyLittleOption::Definitely("123".to_string()), o);
//!
//! // And it will leave variants without an `A` in them alone:
//! let i = MyLittleOption::NotReally;
//! let o = i.map(|value: u8| value.to_string());
//! assert_eq!(MyLittleOption::NotReally, o);
//! # }
//! ```
//!
//! Please note that this derive only maps types of `A`, and will not be able to
//! work on types of eg. `Vec<A>`. You'll have to write your own `Functor`
//! implementation for these.
//!
//! [higher]: https://docs.rs/crate/higher
//! [higher-derive]: https://docs.rs/crate/higher-derive
//! [bartosz]: https://bartoszmilewski.com/2014/10/28/category-theory-for-programmers-the-preface/

#![cfg_attr(not(feature = "std"), no_std)]

mod functor;
pub use crate::functor::Functor;

mod bifunctor;
pub use crate::bifunctor::Bifunctor;

mod profunctor;
pub use crate::profunctor::Profunctor;

mod apply;
pub use crate::apply::Apply;

mod pure;
pub use crate::pure::Pure;

mod applicative;
pub use crate::applicative::Applicative;

mod bind;
pub use crate::bind::Bind;

mod liftm1;
pub use crate::liftm1::LiftM1;

mod ap;
pub use crate::ap::Ap;

mod monad;
pub use crate::monad::Monad;

mod extend;
pub use crate::extend::Extend;

mod extract;
pub use crate::extract::Extract;

mod comonad;
pub use crate::comonad::Comonad;

mod contra;
pub use crate::contra::Contravariant;

mod monoid;
pub use crate::monoid::Monoid;
