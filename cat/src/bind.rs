#[cfg(feature = "std")]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
#[cfg(feature = "std")]
use std::hash::{BuildHasher, Hash};

use higher::Lift;

/// `Bind` lets you chain computations together.
///
/// It takes a function `Fn(A) -> M<B>` and applies it to the `A` inside `M<A>`.
/// You can think of this as a callback function for when the value of `A` is
/// ready to be processed, returning the next computation in the sequence.
pub trait Bind<A, B>: Lift<A, B> {
    /// Use the value inside an `M<A>: Bind` to create an `M<B>`.
    fn bind<F>(self, f: F) -> <Self as Lift<A, B>>::Target1
    where
        F: Fn(A) -> <Self as Lift<A, B>>::Target1;
}

impl<A, B> Bind<A, B> for Option<A> {
    fn bind<F>(self, f: F) -> <Self as Lift<A, B>>::Target1
    where
        F: Fn(A) -> <Self as Lift<A, B>>::Target1,
    {
        self.and_then(f)
    }
}

impl<A, B, E> Bind<A, B> for Result<A, E> {
    fn bind<F>(self, f: F) -> <Self as Lift<A, B>>::Target1
    where
        F: Fn(A) -> <Self as Lift<A, B>>::Target1,
    {
        self.and_then(f)
    }
}

#[cfg(feature = "std")]
impl<A, B> Bind<A, B> for Vec<A> {
    fn bind<F>(self, f: F) -> <Self as Lift<A, B>>::Target1
    where
        F: Fn(A) -> <Self as Lift<A, B>>::Target1,
    {
        self.into_iter().flat_map(|v| f(v).into_iter()).collect()
    }
}

#[cfg(feature = "std")]
impl<A, B> Bind<A, B> for VecDeque<A> {
    fn bind<F>(self, f: F) -> <Self as Lift<A, B>>::Target1
    where
        F: Fn(A) -> <Self as Lift<A, B>>::Target1,
    {
        self.into_iter().flat_map(|v| f(v).into_iter()).collect()
    }
}

#[cfg(feature = "std")]
impl<A, B> Bind<A, B> for LinkedList<A> {
    fn bind<F>(self, f: F) -> <Self as Lift<A, B>>::Target1
    where
        F: Fn(A) -> <Self as Lift<A, B>>::Target1,
    {
        self.into_iter().flat_map(|v| f(v).into_iter()).collect()
    }
}

#[cfg(feature = "std")]
impl<A, B> Bind<A, B> for BinaryHeap<A>
where
    A: Ord,
    B: Ord,
{
    fn bind<F>(self, f: F) -> <Self as Lift<A, B>>::Target1
    where
        F: Fn(A) -> <Self as Lift<A, B>>::Target1,
    {
        self.into_iter().flat_map(|v| f(v).into_iter()).collect()
    }
}

#[cfg(feature = "std")]
impl<A, B> Bind<A, B> for BTreeSet<A>
where
    A: Ord,
    B: Ord,
{
    fn bind<F>(self, f: F) -> <Self as Lift<A, B>>::Target1
    where
        F: Fn(A) -> <Self as Lift<A, B>>::Target1,
    {
        self.into_iter().flat_map(|v| f(v).into_iter()).collect()
    }
}

#[cfg(feature = "std")]
impl<A, B, S> Bind<A, B> for HashSet<A, S>
where
    A: Hash + Eq,
    B: Hash + Eq,
    S: BuildHasher + Default,
{
    fn bind<F>(self, f: F) -> <Self as Lift<A, B>>::Target1
    where
        F: Fn(A) -> <Self as Lift<A, B>>::Target1,
    {
        self.into_iter().flat_map(|v| f(v).into_iter()).collect()
    }
}

#[cfg(feature = "std")]
impl<A, B, C, D, S> Bind<(A, B), (C, D)> for HashMap<A, B, S>
where
    A: Hash + Eq,
    B: Hash + Eq,
    C: Hash + Eq,
    D: Hash + Eq,
    S: BuildHasher + Default,
{
    fn bind<F>(self, f: F) -> <Self as Lift<(A, B), (C, D)>>::Target1
    where
        F: Fn((A, B)) -> <Self as Lift<(A, B), (C, D)>>::Target1,
    {
        self.into_iter().flat_map(|v| f(v).into_iter()).collect()
    }
}

#[cfg(feature = "std")]
impl<A, B, C, D> Bind<(A, B), (C, D)> for BTreeMap<A, B>
where
    A: Ord,
    B: Ord,
    C: Ord,
    D: Ord,
{
    fn bind<F>(self, f: F) -> <Self as Lift<(A, B), (C, D)>>::Target1
    where
        F: Fn((A, B)) -> <Self as Lift<(A, B), (C, D)>>::Target1,
    {
        self.into_iter().flat_map(|v| f(v).into_iter()).collect()
    }
}

#[cfg(all(feature = "std", test))]
mod test {
    use super::*;
    use crate::Pure;
    use proptest::collection::vec;
    use proptest::num::u8;
    use proptest::proptest;

    #[test]
    fn bind_vec() {
        let v = vec![1, 2, 3];
        let o = v.bind(|i| vec![i, i + 1]);
        assert_eq!(vec![1, 2, 2, 3, 3, 4], o);
    }

    proptest! {
        #[test]
        fn identity(v in vec(u8::ANY, 0..1000)) {
            let orig = v.clone();
            let result = v.bind(Pure::pure);
            assert_eq!(orig, result);
        }
    }
}
