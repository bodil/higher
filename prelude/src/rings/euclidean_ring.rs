use super::CommutativeRing;

pub trait EuclideanRing: CommutativeRing {
    fn degree(self) -> usize;
    fn div(self, other: Self) -> Self;
    fn modulo(self, other: Self) -> Self;
}

macro_rules! define_euclidean_ring_int {
    ($type:ty) => {
        impl EuclideanRing for $type {
            fn degree(self) -> usize {
                Self::min(Self::abs(self), Self::MAX) as usize
            }

            fn div(self, other: Self) -> Self {
                self.div_euclid(other)
            }

            fn modulo(self, other: Self) -> Self {
                self.rem_euclid(other)
            }
        }
    };
}

macro_rules! define_euclidean_ring_uint {
    ($type:ty) => {
        impl EuclideanRing for $type {
            fn degree(self) -> usize {
                Self::min(self, Self::MAX) as usize
            }

            fn div(self, other: Self) -> Self {
                self.div_euclid(other)
            }

            fn modulo(self, other: Self) -> Self {
                self.rem_euclid(other)
            }
        }
    };
}

macro_rules! define_euclidean_ring_float {
    ($type:ty) => {
        impl EuclideanRing for $type {
            fn degree(self) -> usize {
                1
            }

            fn div(self, other: Self) -> Self {
                self.div_euclid(other)
            }

            fn modulo(self, _other: Self) -> Self {
                0.0
            }
        }
    };
}

define_euclidean_ring_int!(i8);
define_euclidean_ring_int!(i16);
define_euclidean_ring_int!(i32);
define_euclidean_ring_int!(i64);
define_euclidean_ring_int!(i128);
define_euclidean_ring_int!(isize);
define_euclidean_ring_uint!(u8);
define_euclidean_ring_uint!(u16);
define_euclidean_ring_uint!(u32);
define_euclidean_ring_uint!(u64);
define_euclidean_ring_uint!(u128);
define_euclidean_ring_uint!(usize);

define_euclidean_ring_float!(f32);
define_euclidean_ring_float!(f64);
