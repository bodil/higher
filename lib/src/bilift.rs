use std::collections::{BTreeMap, HashMap};

/// `Bilift` lets you construct a type `T<C, D>` from a type `T<A, B>`.
///
/// If you have a type `T<A, B>` which implements `Biift<A, B, C ,D>`, you can derive the
/// type `T<C, D>` using `<T<A, B> as Bilift<A, B, C, D>>::Target`.
pub trait Bilift<A, B, C, D> {
    type Source;
    type Target;
}

impl<A, B, C, D> Bilift<A, B, C, D> for Result<A, B> {
    type Source = Self;
    type Target = Result<C, D>;
}

impl<A, B, C, D, S> Bilift<A, B, C, D> for HashMap<A, B, S> {
    type Source = Self;
    type Target = HashMap<C, D, S>;
}

impl<A, B, C, D> Bilift<A, B, C, D> for BTreeMap<A, B> {
    type Source = Self;
    type Target = BTreeMap<C, D>;
}
