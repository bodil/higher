use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use std::hash::{BuildHasher, Hash};

use crate::Lift;

/// A `Functor` defines a method `map` on a type `F<_>: Functor` which converts
/// an `F<A>` to `F<B>` using a function `Fn(A) -> B`.
pub trait Functor<A, B>: Lift<A, B> {
    fn map<F>(self, f: F) -> <Self as Lift<A, B>>::Target1
    where
        F: Fn(A) -> B;
}

impl<A, B> Functor<A, B> for Option<A> {
    fn map<F>(self, f: F) -> <Self as Lift<A, B>>::Target1
    where
        F: Fn(A) -> B,
    {
        self.map(f)
    }
}

impl<A, B, E> Functor<A, B> for Result<A, E> {
    fn map<F>(self, f: F) -> <Self as Lift<A, B>>::Target1
    where
        F: Fn(A) -> B,
    {
        self.map(f)
    }
}

impl<A, B> Functor<A, B> for Vec<A> {
    fn map<F>(self, f: F) -> <Self as Lift<A, B>>::Target1
    where
        F: Fn(A) -> B,
    {
        self.into_iter().map(f).collect()
    }
}

impl<A, B, S> Functor<A, B> for HashSet<A, S>
where
    A: Hash + Eq,
    B: Hash + Eq,
    S: BuildHasher + Default,
{
    fn map<F>(self, f: F) -> <Self as Lift<A, B>>::Target1
    where
        F: Fn(A) -> B,
    {
        self.into_iter().map(f).collect()
    }
}

impl<A, B> Functor<A, B> for BTreeSet<A>
where
    A: Ord,
    B: Ord,
{
    fn map<F>(self, f: F) -> <Self as Lift<A, B>>::Target1
    where
        F: Fn(A) -> B,
    {
        self.into_iter().map(f).collect()
    }
}

impl<A, B, C, D, S> Functor<(A, B), (C, D)> for HashMap<A, B, S>
where
    A: Hash + Eq,
    B: Hash + Eq,
    C: Hash + Eq,
    D: Hash + Eq,
    S: BuildHasher + Default,
{
    fn map<F>(self, f: F) -> <Self as Lift<(A, B), (C, D)>>::Target1
    where
        F: Fn((A, B)) -> (C, D),
    {
        self.into_iter().map(f).collect()
    }
}

impl<A, B, C, D> Functor<(A, B), (C, D)> for BTreeMap<A, B>
where
    A: Ord,
    B: Ord,
    C: Ord,
    D: Ord,
{
    fn map<F>(self, f: F) -> <Self as Lift<(A, B), (C, D)>>::Target1
    where
        F: Fn((A, B)) -> (C, D),
    {
        self.into_iter().map(f).collect()
    }
}

impl<A, B> Functor<A, B> for VecDeque<A> {
    fn map<F>(self, f: F) -> <Self as Lift<A, B>>::Target1
    where
        F: Fn(A) -> B,
    {
        self.into_iter().map(f).collect()
    }
}

impl<A, B> Functor<A, B> for LinkedList<A> {
    fn map<F>(self, f: F) -> <Self as Lift<A, B>>::Target1
    where
        F: Fn(A) -> B,
    {
        self.into_iter().map(f).collect()
    }
}

impl<A, B> Functor<A, B> for BinaryHeap<A>
where
    A: Ord,
    B: Ord,
{
    fn map<F>(self, f: F) -> <Self as Lift<A, B>>::Target1
    where
        F: Fn(A) -> B,
    {
        self.into_iter().map(f).collect()
    }
}
