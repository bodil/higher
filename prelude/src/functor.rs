use std::{
    cell::RefCell,
    collections::{LinkedList, VecDeque},
    mem::MaybeUninit,
    rc::Rc,
};

use crate::repeat;

/// A `Functor` lets you change the type parameter of a generic type.
///
/// A `Functor` defines a method `fmap` on a type `F<_>: Functor` which converts
/// an `F<A>` to `F<B>` using a function `Fn(A) -> B` applied to the `A`s inside
/// it.
///
/// You can also use this just to modify the values inside your container value
/// without changing their type, if the mapping function returns a value of the
/// same type.  This is called an "endofunctor." In an ideal Rust, we would be
/// able to implement this as a special case of [`fmap`](Functor::fmap)
/// modifying the data in place, but in the Rust we have, beware that using
/// [`fmap`](Functor::fmap) in this manner is considerably less efficient than
/// using a mutable reference iterator.
pub trait Functor<'a, A: 'a> {
    type Target<T: 'a>;

    /// Map a functor of `A` to a functor of `B` using a function from `A`
    /// to `B`.
    fn fmap<B: 'a, F: 'a>(self, f: F) -> Self::Target<B>
    where
        F: Fn(A) -> B;

    /// Map the functor to the provided constant value.
    fn fconst<B>(self, b: B) -> Self::Target<B>
    where
        Self: Sized,
        B: Clone,
    {
        self.fmap(repeat(b))
    }

    /// Map the functor to the unit value `()`.
    fn void(self) -> Self::Target<()>
    where
        Self: Sized,
    {
        self.fconst(())
    }

    /// Turn the functor into an iterator.
    ///
    /// ```
    /// # use higher::Functor;
    /// let my_functor = vec![1, 2, 3];
    /// let iter = my_functor.f_into_iter();
    /// let my_vec: Vec<i32> = iter.collect();
    /// assert_eq!(my_vec, vec![1, 2, 3]);
    /// ```
    fn f_into_iter(self) -> Box<dyn Iterator<Item = A>>
    where
        Self: Sized,
        A: 'static,
    {
        let store = Rc::new(RefCell::new(Vec::new()));
        let istore = store.clone();
        self.fmap(move |a| istore.borrow_mut().push(a));
        Box::new(
            match Rc::try_unwrap(store) {
                Ok(store) => store,
                Err(_) => unreachable!(),
            }
            .into_inner()
            .into_iter(),
        )
    }
}

/// `FunctorRef` is an extension to [`Functor`](Functor) which provides a
/// non-destructive [`fmap`](Functor::fmap) passing references to the mapping
/// function.
///
/// This trait is separate from [`Functor`](Functor) because it can only be
/// implemented for types which can be reconstructed using only references and
/// the function mapping `&A` to `B`. For instance, it can't be implemented for
/// [`Result<A, E>`](Result) because in the [`Err`](Result::Err) case, we can't
/// map to another [`Err`](Result::Err) without ownership of the `E`.
pub trait FunctorRef<'a, A: 'a>: Functor<'a, A> {
    /// Map a functor of `A` to a functor of `B` using a function from `&A` to
    /// `B`.
    fn fmap_ref<B: 'a, F: 'a>(&self, f: F) -> Self::Target<B>
    where
        F: Fn(&A) -> B;

    /// Given a type `A` implementing [`Clone`](Clone), create a new identical
    /// `FunctorRef<A>` by cloning the values inside `self`.
    ///
    /// This is mostly useful for data structures which don't necessarily
    /// implement [`Clone`](Clone). For those which do, you should reimplement
    /// this method simply as a call to [`Clone::clone()`](Clone) for
    /// performance.
    fn fclone(&self) -> Self
    where
        Self: Sized + FunctorRef<'a, A, Target<A> = Self>,
        A: Clone,
    {
        self.fmap_ref(Clone::clone)
    }
}

impl<'a, A: 'a> Functor<'a, A> for Option<A> {
    type Target<T: 'a> = Option<T>;

    fn fmap<B: 'a, F>(self, f: F) -> Self::Target<B>
    where
        F: Fn(A) -> B,
    {
        self.map(f)
    }
}

impl<'a, A: 'a> FunctorRef<'a, A> for Option<A> {
    fn fmap_ref<B: 'a, F>(&self, f: F) -> Self::Target<B>
    where
        F: Fn(&A) -> B,
    {
        self.as_ref().map(f)
    }

    fn fclone(&self) -> Self
    where
        A: Clone,
    {
        self.clone()
    }
}

impl<'a, A: 'a, E> Functor<'a, A> for Result<A, E> {
    type Target<T: 'a> = Result<T, E>;

    fn fmap<B, F>(self, f: F) -> Self::Target<B>
    where
        B: 'a,
        F: Fn(A) -> B,
    {
        self.map(f)
    }
}

impl<'a, A: 'a, const N: usize> Functor<'a, A> for [A; N] {
    type Target<T: 'a> = [T; N];

    #[allow(unsafe_code)]
    fn fmap<B, F>(self, f: F) -> Self::Target<B>
    where
        B: 'a,
        F: Fn(A) -> B + 'a,
    {
        let mut out: MaybeUninit<[B; N]> = MaybeUninit::uninit();
        let mut ptr: *mut B = out.as_mut_ptr().cast();
        for item in self.into_iter() {
            unsafe {
                ptr.write(f(item));
                ptr = ptr.add(1);
            }
        }
        unsafe { out.assume_init() }
    }
}

impl<'a, A: 'a, const N: usize> FunctorRef<'a, A> for [A; N] {
    #[allow(unsafe_code)]
    fn fmap_ref<B, F>(&self, f: F) -> Self::Target<B>
    where
        B: 'a,
        F: Fn(&A) -> B + 'a,
    {
        let mut out: MaybeUninit<[B; N]> = MaybeUninit::uninit();
        let mut ptr: *mut B = out.as_mut_ptr().cast();
        for item in self.into_iter() {
            unsafe {
                ptr.write(f(item));
                ptr = ptr.add(1);
            }
        }
        unsafe { out.assume_init() }
    }

    fn fclone(&self) -> Self
    where
        A: Clone,
    {
        self.clone()
    }
}

macro_rules! impl_functor_for_collection {
    ($type:ident) => {
        impl<'a, A: 'a> Functor<'a, A> for $type<A> {
            type Target<T: 'a> = $type<T>;

            fn fmap<B, F>(self, f: F) -> Self::Target<B>
            where
                B: 'a,
                F: Fn(A) -> B,
            {
                self.into_iter().map(f).collect()
            }
        }

        impl<'a, A: 'a> FunctorRef<'a, A> for $type<A> {
            fn fmap_ref<B: 'a, F>(&self, f: F) -> Self::Target<B>
            where
                F: Fn(&A) -> B,
            {
                self.iter().map(f).collect()
            }

            fn fclone(&self) -> Self
            where
                A: Clone,
            {
                self.clone()
            }
        }
    };
}

impl_functor_for_collection!(Vec);
impl_functor_for_collection!(VecDeque);
impl_functor_for_collection!(LinkedList);

#[cfg(test)]
mod test {
    use crate::Functor;

    #[test]
    fn option_functor() {
        let a = Option::Some(31337);
        let b = a.fmap(|x| x + 2);
        assert_eq!(b, Option::Some(31339));
    }

    #[test]
    fn array_endofunctor() {
        let a: [usize; 5] = [1, 2, 3, 4, 5];
        let b = a.fmap(|x| x * 2);
        assert_eq!(b, [2, 4, 6, 8, 10]);
    }

    #[test]
    fn array_exofunctor() {
        let a: [u64; 5] = [1, 2, 3, 4, 5];
        let b = a.fmap(|x| ((x * 2) as u16));
        assert_eq!(b, [2, 4, 6, 8, 10]);
    }

    #[test]
    fn vec_endofunctor() {
        let a = vec![1, 2, 3, 4, 5];
        let b = a.fmap(|x| x * 2);
        assert_eq!(b, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn vec_exofunctor() {
        let a = vec![1, 2, 3];
        let b = a.fmap(|x| (x as usize) * 2);
        assert_eq!(b, vec![2usize, 4usize, 6usize]);
    }
}
