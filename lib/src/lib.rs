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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flatmap_that_shit() {
        assert_eq!(
            Some("1337".to_string()),
            Some(1337).bind(|n| Some(format!("{}", n)))
        );
        assert_eq!(vec![1, 1, 2, 2, 3, 3], vec![1, 2, 3].bind(|v| vec![v, v]))
    }
}
