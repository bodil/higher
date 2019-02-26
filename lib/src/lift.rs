use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

/// `Lift` lets you construct a type `T<B>` from a type `T<A>`.
///
/// If you have a type `T<A>` which implements `Lift<A, B>`, you can derive the
/// type `T<B>` using `<T<A> as Lift<A, B>>::Target1`.
///
/// The type `T<A>` is also available as the `Source` associated type, if you
/// should need it.
pub trait Lift<A, B> {
    type Source;
    type Target1;
}

/// `Lift3` extends `Lift` to let you construct two types `T<B>` and `T<C>` from
/// a type `T<A>`.
///
/// `T<B>` can be found at `<T<A> as Lift3<A, B, C>>::Target2`, and `T<C>` at
/// `<T<A> as Lift<A, B>>::Target1`.
///
/// The naming convention is that the `Target`s are numbered from right to left.
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
    type Source = Self;
    type Target1 = Option<B>;
}

impl<A, B, E> Lift<A, B> for Result<A, E> {
    type Source = Self;
    type Target1 = Result<B, E>;
}

impl<A, B> Lift<A, B> for Vec<A> {
    type Source = Self;
    type Target1 = Vec<B>;
}

impl<A, B, S> Lift<A, B> for HashSet<A, S> {
    type Source = Self;
    type Target1 = HashSet<B, S>;
}

impl<A, B> Lift<A, B> for BTreeSet<A> {
    type Source = Self;
    type Target1 = BTreeSet<B>;
}

impl<A, B, C, D, S> Lift<(A, B), (C, D)> for HashMap<A, B, S> {
    type Source = Self;
    type Target1 = HashMap<C, D, S>;
}

impl<A, B, C, D> Lift<(A, B), (C, D)> for BTreeMap<A, B> {
    type Source = Self;
    type Target1 = BTreeMap<C, D>;
}

impl<A, B> Lift<A, B> for VecDeque<A> {
    type Source = Self;
    type Target1 = VecDeque<B>;
}

impl<A, B> Lift<A, B> for LinkedList<A> {
    type Source = Self;
    type Target1 = LinkedList<B>;
}

impl<A, B> Lift<A, B> for BinaryHeap<A> {
    type Source = Self;
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
