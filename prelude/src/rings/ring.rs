use super::Semiring;

pub trait Ring: Semiring {
    fn sub(self, other: Self) -> Self;
}

impl Ring for () {
    fn sub(self, _other: Self) -> Self {
        ()
    }
}

macro_rules! define_ring {
    ($type:ty) => {
        impl Ring for $type {
            fn sub(self, other: Self) -> Self {
                self - other
            }
        }
    };
}

define_ring!(i8);
define_ring!(i16);
define_ring!(i32);
define_ring!(i64);
define_ring!(i128);
define_ring!(isize);
define_ring!(u8);
define_ring!(u16);
define_ring!(u32);
define_ring!(u64);
define_ring!(u128);
define_ring!(usize);
define_ring!(f32);
define_ring!(f64);
