#![deny(unsafe_code, nonstandard_style)]
#![forbid(rust_2018_idioms)]
#![warn(unreachable_pub, missing_debug_implementations)]

pub mod effect;
use higher::Functor;

#[doc(inline)]
pub use crate::effect::Effect;

pub mod io;
#[doc(inline)]
pub use crate::io::IO;

#[derive(Functor)]
struct Welp<A>(A);
