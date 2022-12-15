/// A `Semigroup` is a type with an associative operation. In plain terms, this
/// means you can take two values of this type and add them together into a
/// different value of the same type. The most obvious example of this is
/// addition of numbers: `2 + 2 = 4`, another is string concatenation: `"Hello "
/// + "Joe" = "Hello Joe"`.
///
/// Semigroups must follow the law of associativity:
/// * `(x + y) + z = x + (y + z)`
///
/// A `Semigroup` differs from `std::ops::Add` in that `Add` can be defined
/// for any collection of types, eg. you could define `Add` for a type `A` which
/// takes a second argument of type `B` and returns a third type `C`, whereas a
/// `Semigroup` only deals with a single type `A`.
pub trait Semigroup {
    fn mappend(self, other: Self) -> Self;
}

#[cfg(feature = "std")]
impl<A> Semigroup for Vec<A> {
    fn mappend(mut self, other: Self) -> Self {
        self.extend(other);
        self
    }
}

#[cfg(feature = "std")]
impl Semigroup for String {
    fn mappend(self, other: Self) -> Self {
        self + &other
    }
}

macro_rules! define_semigroup {
    ($type:ty) => {
        impl Semigroup for $type {
            fn mappend(self, other: Self) -> Self {
                self + other
            }
        }
    };
}

define_semigroup!(i8);
define_semigroup!(i16);
define_semigroup!(i32);
define_semigroup!(i64);
define_semigroup!(i128);
define_semigroup!(isize);
define_semigroup!(u8);
define_semigroup!(u16);
define_semigroup!(u32);
define_semigroup!(u64);
define_semigroup!(u128);
define_semigroup!(usize);
