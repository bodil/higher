pub trait Semiring {
    const ZERO: Self;
    const ONE: Self;

    fn add(self, other: Self) -> Self;
    fn mul(self, other: Self) -> Self;
}

impl Semiring for () {
    const ZERO: Self = ();

    const ONE: Self = ();

    fn add(self, _other: Self) -> Self {
        ()
    }

    fn mul(self, _other: Self) -> Self {
        ()
    }
}

macro_rules! define_semiring_int {
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

define_semiring_int!(i8);
define_semiring_int!(i16);
define_semiring_int!(i32);
define_semiring_int!(i64);
define_semiring_int!(i128);
define_semiring_int!(isize);
define_semiring_int!(u8);
define_semiring_int!(u16);
define_semiring_int!(u32);
define_semiring_int!(u64);
define_semiring_int!(u128);
define_semiring_int!(usize);

macro_rules! define_semiring_float {
    ($type:ty) => {
        impl Semiring for $type {
            const ZERO: Self = 0.0;
            const ONE: Self = 1.0;
            fn add(self, other: Self) -> Self {
                self + other
            }
            fn mul(self, other: Self) -> Self {
                self * other
            }
        }
    };
}

define_semiring_float!(f32);
define_semiring_float!(f64);
