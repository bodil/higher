use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use std::hash::{BuildHasher, Hash};
use std::iter;

/// `Pure` lets you construct a value of type `F<A>` using a single value of
/// `A`.
pub trait Pure<A> {
    fn pure(value: A) -> Self;
}

impl<A> Pure<A> for Option<A> {
    fn pure(value: A) -> Self {
        Some(value)
    }
}

impl<A, E> Pure<A> for Result<A, E> {
    fn pure(value: A) -> Self {
        Ok(value)
    }
}

impl<A> Pure<A> for Vec<A> {
    fn pure(value: A) -> Self {
        vec![value]
    }
}

impl<A> Pure<A> for VecDeque<A> {
    fn pure(value: A) -> Self {
        iter::once(value).collect()
    }
}

impl<A> Pure<A> for LinkedList<A> {
    fn pure(value: A) -> Self {
        iter::once(value).collect()
    }
}

impl<A> Pure<A> for BinaryHeap<A>
where
    A: Ord,
{
    fn pure(value: A) -> Self {
        iter::once(value).collect()
    }
}

impl<A> Pure<A> for BTreeSet<A>
where
    A: Ord,
{
    fn pure(value: A) -> Self {
        iter::once(value).collect()
    }
}

impl<A, S> Pure<A> for HashSet<A, S>
where
    A: Hash + Eq,
    S: BuildHasher + Default,
{
    fn pure(value: A) -> Self {
        iter::once(value).collect()
    }
}

impl<A, B> Pure<(A, B)> for BTreeMap<A, B>
where
    A: Ord,
    B: Ord,
{
    fn pure(value: (A, B)) -> Self {
        iter::once(value).collect()
    }
}

impl<A, B, S> Pure<(A, B)> for HashMap<A, B, S>
where
    A: Hash + Eq,
    B: Hash + Eq,
    S: BuildHasher + Default,
{
    fn pure(value: (A, B)) -> Self {
        iter::once(value).collect()
    }
}
