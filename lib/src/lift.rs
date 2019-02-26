use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

pub trait Lift<A, B> {
    type Target1;
}

pub trait Lift3<A, B, C>: Lift<A, C> {
    type Target2;
    fn cast(
        from: <<Self as Lift3<A, B, C>>::Target2 as Lift<B, C>>::Target1,
    ) -> <Self as Lift<A, C>>::Target1
    where
        <Self as Lift3<A, B, C>>::Target2: Lift<B, C>,
    {
        crate::unsafe_coerce(from)
    }
}

impl<A, B> Lift<A, B> for Option<A> {
    type Target1 = Option<B>;
}

impl<A, B, E> Lift<A, B> for Result<A, E> {
    type Target1 = Result<B, E>;
}

impl<A, B> Lift<A, B> for Vec<A> {
    type Target1 = Vec<B>;
}

impl<A, B, S> Lift<A, B> for HashSet<A, S> {
    type Target1 = HashSet<B, S>;
}

impl<A, B> Lift<A, B> for BTreeSet<A> {
    type Target1 = BTreeSet<B>;
}

impl<A, B, C, D, S> Lift<(A, B), (C, D)> for HashMap<A, B, S> {
    type Target1 = HashMap<C, D, S>;
}

impl<A, B, C, D> Lift<(A, B), (C, D)> for BTreeMap<A, B> {
    type Target1 = BTreeMap<C, D>;
}

impl<A, B> Lift<A, B> for VecDeque<A> {
    type Target1 = VecDeque<B>;
}

impl<A, B> Lift<A, B> for LinkedList<A> {
    type Target1 = LinkedList<B>;
}

impl<A, B> Lift<A, B> for BinaryHeap<A> {
    type Target1 = BinaryHeap<B>;
}

impl<A, B, C> Lift3<A, B, C> for Option<A> {
    type Target2 = Option<B>;
}

impl<A, B, C, E> Lift3<A, B, C> for Result<A, E> {
    type Target2 = Result<B, E>;
}

impl<A, B, C> Lift3<A, B, C> for Vec<A> {
    type Target2 = Vec<B>;
}

impl<A, B, C, S> Lift3<A, B, C> for HashSet<A, S> {
    type Target2 = HashSet<B, S>;
}

impl<A, B, C> Lift3<A, B, C> for BTreeSet<A> {
    type Target2 = BTreeSet<B>;
}

impl<A, B, C> Lift3<A, B, C> for VecDeque<A> {
    type Target2 = VecDeque<B>;
}

impl<A, B, C> Lift3<A, B, C> for LinkedList<A> {
    type Target2 = LinkedList<B>;
}

impl<A, B, C> Lift3<A, B, C> for BinaryHeap<A> {
    type Target2 = BinaryHeap<B>;
}
