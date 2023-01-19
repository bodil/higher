use core::ops::{Deref, DerefMut};

use crate::{algebras::HeytingAlgebra, rings::Semiring, Semigroup};

/// A `Monoid` consists of a [`Semigroup`](Semigroup) and an empty value (the
/// [`Default`](Default) trait) plus the following laws:
///
/// - Associativity: `(x + y) + z == x + (y + z)`
/// - Identity: `0 + a == a + 0 == a`
pub trait Monoid: Semigroup + Default {}

impl<A> Monoid for A where A: Semigroup + Default {}

macro_rules! deref_impl {
    ($type:ident) => {
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

deref_impl!(Additive);

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

deref_impl!(Multiplicative);

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

deref_impl!(Conj);

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

deref_impl!(Disj);

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

deref_impl!(Dual);

impl<A> Semigroup for Dual<A>
where
    A: Semigroup,
{
    fn mappend(self, other: Self) -> Self {
        Self(other.0.mappend(self.0))
    }
}
