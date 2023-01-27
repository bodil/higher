#![deny(unsafe_code, nonstandard_style)]
#![forbid(rust_2018_idioms)]
#![warn(unreachable_pub, missing_debug_implementations)]

//! Effect monads for [`higher`](https://docs.rs/higher).
//!
//! These are of limited usefulness, being single-threaded out of necessity
//! because of our inability to specify [`Send`](Send) bounds, but they serve as
//! a good example of effects with monadic abstractions.

pub mod effect;
#[doc(inline)]
pub use crate::effect::Effect;

pub mod io;
#[doc(inline)]
pub use crate::io::IO;
