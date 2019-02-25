use std::collections::{BTreeMap, HashMap};

pub trait Bilift<A, B, C, D> {
    type Target;
}

impl<A, B, C, D> Bilift<A, B, C, D> for Result<A, B> {
    type Target = Result<C, D>;
}

impl<A, B, C, D, S> Bilift<A, B, C, D> for HashMap<A, B, S> {
    type Target = HashMap<C, D, S>;
}

impl<A, B, C, D> Bilift<A, B, C, D> for BTreeMap<A, B> {
    type Target = BTreeMap<C, D>;
}
