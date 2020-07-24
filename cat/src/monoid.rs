use core::ops::Add;

/// A `Monoid` consists of a semigroup (the [`Add`][Add] trait in Rust) and an
/// empty value (the [`Default`][Default] trait) plus the following laws:
///
/// - Associativity: `(x + y) + z == x + (y + z)`
/// - Identity: `0 + a == a + 0 == a`
///
/// [Add]: https://doc.rust-lang.org/core/ops/trait.Add.html
/// [Default]: https://doc.rust-lang.org/core/default/trait.Default.html
pub trait Monoid: Add + Default {}
