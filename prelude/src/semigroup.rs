use core::{
    convert::Infallible,
    ops::{Deref, DerefMut},
};

use crate::Functor;

/// A `Semigroup` is a type with an associative operation. In plain terms, this
/// means you can take two values of this type and add them together into a
/// different value of the same type. The most obvious example of this is
/// addition of numbers: `2 + 2 = 4`, another is string concatenation:
/// `"Hello " + "Joe" = "Hello Joe"`.
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

impl Semigroup for () {
    fn mappend(self, _other: Self) -> Self {}
}

impl Semigroup for Infallible {
    fn mappend(self, _other: Self) -> Self {
        unreachable!()
    }
}

/// Semigroup where [`mappend`](Semigroup::mappend) always takes the first option.
///
/// ```
/// # use higher::semigroup::{Semigroup, First};
/// # let x = 5;
/// # let y = 8;
/// # assert!(
/// First(x).mappend(First(y)) == First(x)
/// # );
/// ```
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct First<A>(pub A);

impl<A> Deref for First<A> {
    type Target = A;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<A> DerefMut for First<A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<A> Semigroup for First<A> {
    fn mappend(self, _other: Self) -> Self {
        self
    }
}

/// Semigroup where [`mappend`](Semigroup::mappend) always takes the last option.
///
/// ```
/// # use higher::semigroup::{Semigroup, Last};
/// # let x = 5;
/// # let y = 8;
/// # assert!(
/// Last(x).mappend(Last(y)) == Last(y)
/// # );
/// ```
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Last<A>(pub A);

impl<A> Deref for Last<A> {
    type Target = A;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<A> DerefMut for Last<A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<A> Semigroup for Last<A> {
    fn mappend(self, other: Self) -> Self {
        other
    }
}
