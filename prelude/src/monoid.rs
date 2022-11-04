use crate::Semigroup;

/// A `Monoid` consists of a `Semigroup` and an empty value (the
/// [`Default`][Default] trait) plus the following laws:
///
/// - Associativity: `(x + y) + z == x + (y + z)`
/// - Identity: `0 + a == a + 0 == a`
///
/// [Default]: https://doc.rust-lang.org/std/default/trait.Default.html
pub trait Monoid: Semigroup + Default {}

impl<A> Monoid for A where A: Semigroup + Default {}
