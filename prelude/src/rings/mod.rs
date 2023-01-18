pub mod semiring;
#[doc(inline)]
pub use self::semiring::Semiring;

pub mod ring;
#[doc(inline)]
pub use self::ring::Ring;

pub mod commutative_ring;
#[doc(inline)]
pub use self::commutative_ring::CommutativeRing;

pub mod euclidean_ring;
#[doc(inline)]
pub use self::euclidean_ring::EuclideanRing;

pub mod division_ring;
#[doc(inline)]
pub use self::division_ring::DivisionRing;

pub mod field;
#[doc(inline)]
pub use self::field::Field;
