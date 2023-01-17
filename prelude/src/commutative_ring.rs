use crate::Ring;

pub trait CommutativeRing: Ring {}

macro_rules! define_commutative_ring {
    ($type:ty) => {
        impl CommutativeRing for $type {}
    };
}

define_commutative_ring!(());
define_commutative_ring!(i8);
define_commutative_ring!(i16);
define_commutative_ring!(i32);
define_commutative_ring!(i64);
define_commutative_ring!(i128);
define_commutative_ring!(isize);
define_commutative_ring!(u8);
define_commutative_ring!(u16);
define_commutative_ring!(u32);
define_commutative_ring!(u64);
define_commutative_ring!(u128);
define_commutative_ring!(usize);
define_commutative_ring!(f32);
define_commutative_ring!(f64);
