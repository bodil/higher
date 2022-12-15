pub trait Semiring {
    const ZERO: Self;
    const ONE: Self;

    fn add(self, other: Self) -> Self;
    fn mul(self, other: Self) -> Self;
}

macro_rules! define_semiring {
    ($type:ty) => {
        impl Semiring for $type {
            const ZERO: Self = 0;
            const ONE: Self = 1;
            fn add(self, other: Self) -> Self {
                self + other
            }
            fn mul(self, other: Self) -> Self {
                self * other
            }
        }
    };
}

define_semiring!(i8);
define_semiring!(i16);
define_semiring!(i32);
define_semiring!(i64);
define_semiring!(i128);
define_semiring!(isize);
define_semiring!(u8);
define_semiring!(u16);
define_semiring!(u32);
define_semiring!(u64);
define_semiring!(u128);
define_semiring!(usize);
