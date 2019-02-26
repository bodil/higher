mod lift;
pub use crate::lift::{Lift, Lift3};

mod bilift;
pub use crate::bilift::Bilift;

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

/// You'd better be sure about what you're doing before using this.
pub(crate) fn unsafe_coerce<A, B>(mut a: A) -> B {
    unsafe {
        let ptr = &mut a as *mut _ as *mut B;
        let out = ::std::ptr::read(ptr);
        ::std::mem::forget(a);
        out
    }
}
