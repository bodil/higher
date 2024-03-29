use std::ops::{Deref, DerefMut};

use crate::{algebras::HeytingAlgebra, rings::Semiring, Semigroup};

/// A `Monoid` consists of a [`Semigroup`](Semigroup) and an empty value (the
/// [`Default`](Default) trait) plus the following laws:
///
/// - Associativity: `(x + y) + z == x + (y + z)`
/// - Identity: `0 + a == a + 0 == a`
///
/// If you're wondering why this isn't implemented for Rust's primitive
/// integers, it's because they're essentially two `Monoid`s in a trenchcoat:
/// both addition and multiplication fulfill the monoid laws. To distinguish
/// between the two, you can wrap them in the [`Additive`](Additive) and
/// [`Multiplicative`](Multiplicative) newtypes. It's generally easier to use
/// [`Semiring`](Semiring) as an abstraction for numbers for this
/// reason, but the newtypes are there if you just need to use a number as a
/// monoid.
pub trait Monoid: Semigroup + Default {}

impl<A> Monoid for A where A: Semigroup + Default {}

macro_rules! impl_newtype {
    ($type:ident) => {
        impl<A> $type<A> {
            pub fn unwrap(self) -> A {
                self.0
            }
        }

        impl<A> From<A> for $type<A> {
            fn from(value: A) -> Self {
                Self(value)
            }
        }

        impl<A> Deref for $type<A> {
            type Target = A;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<A> DerefMut for $type<A> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

/// Monoid and semigroup for semirings under addition.
///
/// Wrap a [`Semiring`](Semiring) (such as an integer or float) in this to use
/// its addition method as a monoid.
///
/// ```
/// # use higher::{Semigroup, rings::Semiring, monoid::Additive};
/// # let x = 5;
/// # let y = 8;
/// # assert!(
/// Additive(x).mappend(Additive(y)) == Additive(x + y)
/// # );
/// # assert!(
/// Additive::<u32>::default() == Additive(<u32>::ZERO)
/// # );
/// ```
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Additive<A>(pub A);

impl_newtype!(Additive);

impl<A> Default for Additive<A>
where
    A: Semiring,
{
    fn default() -> Self {
        Self(A::ZERO)
    }
}

impl<A> Semigroup for Additive<A>
where
    A: Semiring,
{
    fn mappend(self, other: Self) -> Self {
        Self(self.0.add(other.0))
    }
}

/// Monoid and semigroup for semirings under multiplication.
///
/// Wrap a [`Semiring`](Semiring) (such as an integer or float) in this to use
/// its multiplication method as a monoid.
///
/// ```
/// # use higher::{Semigroup, rings::Semiring, monoid::Multiplicative};
/// # let x = 5;
/// # let y = 8;
/// # assert!(
/// Multiplicative(x).mappend(Multiplicative(y)) == Multiplicative(x * y)
/// # );
/// # assert!(
/// Multiplicative::<u32>::default() == Multiplicative(<u32>::ONE)
/// # );
/// ```
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Multiplicative<A>(pub A);

impl_newtype!(Multiplicative);

impl<A> Default for Multiplicative<A>
where
    A: Semiring,
{
    fn default() -> Self {
        Self(A::ONE)
    }
}

impl<A> Semigroup for Multiplicative<A>
where
    A: Semiring,
{
    fn mappend(self, other: Self) -> Self {
        Self(self.0.mul(other.0))
    }
}

/// Monoid and semigroup for conjunction.
///
/// Wrap a [`HeytingAlgebra`](HeytingAlgebra) (such as a boolean) in this to use
/// its conjunction method (logical "and") as a monoid.
///
/// ```
/// # use higher::{Semigroup, monoid::Conj, algebras::HeytingAlgebra};
/// # let x = true;
/// # let y = false;
/// # assert!(
/// Conj(x).mappend(Conj(y)) == Conj(x.conj(y))
/// # ); assert!(
/// Conj::<bool>::default() == Conj(bool::TRUE)
/// # );
/// ```
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Conj<A>(pub A);

impl_newtype!(Conj);

impl<A> Default for Conj<A>
where
    A: HeytingAlgebra,
{
    fn default() -> Self {
        Self(A::TRUE)
    }
}

impl<A> Semigroup for Conj<A>
where
    A: HeytingAlgebra,
{
    fn mappend(self, other: Self) -> Self {
        Self(self.0.conj(other.0))
    }
}

impl<A> Semiring for Conj<A>
where
    A: HeytingAlgebra,
{
    const ZERO: Self = Self(A::TRUE);

    const ONE: Self = Self(A::FALSE);

    fn add(self, other: Self) -> Self {
        Self(self.0.conj(other.0))
    }

    fn mul(self, other: Self) -> Self {
        Self(self.0.disj(other.0))
    }
}

/// Monoid and semigroup for disjunction.
///
/// Wrap a [`HeytingAlgebra`](HeytingAlgebra) (such as a boolean) in this to use
/// its disjunction method (logical "or") as a monoid.
///
/// ```
/// # use higher::{Semigroup, monoid::Disj, algebras::HeytingAlgebra};
/// # let x = true;
/// # let y = false;
/// # assert!(
/// Disj(x).mappend(Disj(y)) == Disj(x.disj(y))
/// # ); assert!(
/// Disj::<bool>::default() == Disj(bool::FALSE)
/// # );
/// ```
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Disj<A>(pub A);

impl_newtype!(Disj);

impl<A> Default for Disj<A>
where
    A: HeytingAlgebra,
{
    fn default() -> Self {
        Self(A::FALSE)
    }
}

impl<A> Semigroup for Disj<A>
where
    A: HeytingAlgebra,
{
    fn mappend(self, other: Self) -> Self {
        Self(self.0.disj(other.0))
    }
}

impl<A> Semiring for Disj<A>
where
    A: HeytingAlgebra,
{
    const ZERO: Self = Self(A::FALSE);

    const ONE: Self = Self(A::TRUE);

    fn add(self, other: Self) -> Self {
        Self(self.0.disj(other.0))
    }

    fn mul(self, other: Self) -> Self {
        Self(self.0.conj(other.0))
    }
}

/// The dual of a monoid.
///
/// This just [`mappend`](Semigroup::mappend)s semigroups in reverse order.
///
/// ```
/// # use higher::{Semigroup, monoid::{Dual, Additive}};
/// # let x = Additive(5);
/// # let y = Additive(8);
/// # assert!(
/// Dual(x).mappend(Dual(y)) == Dual(y.mappend(x))
/// # ); assert!(
/// Dual::<String>::default() == Dual(String::default())
/// # );
/// ```
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Dual<A>(pub A);

impl_newtype!(Dual);

impl<A> Semigroup for Dual<A>
where
    A: Semigroup,
{
    fn mappend(self, other: Self) -> Self {
        Self(other.0.mappend(self.0))
    }
}
