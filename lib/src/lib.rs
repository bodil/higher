//! This crate implements a mechanism to fake higher kinded types in a limited
//! way in Rust with a minimum of boilerplate.
//!
//! # But Why?
//!
//! If you ever had a trait `T<A>` and wanted to swap the type parameter out in
//! a generic way to produce a type `T<B>` in the trait's type signatures,
//! there's no straightforward way to do this currently in Rust, because this
//! would require Rust's type system to support higher kinded types. It can deal
//! with `T<A>` but it can't deal with the concept of just `T` where the type
//! parameter is yet unspecified (what's known as a type constructor - think of
//! it as a function which takes a type parameter, like `A`, and returns a
//! concrete type `T<A>`).
//!
//! `higher` provides the trait `Lift` which, when given a type `T<A>` which
//! implements `Lift<A, B>`, will let you derive the concrete type `T<B>`. This
//! needs to be implemented for any `T<A>` which needs `Lift`ing, but the
//! [`higher-derive`][higher-derive] crate provides a custom derive for it, so
//! that you can quickly add it to your own types without boilerplate:
//!
//! ```nocompile
//! #[derive(Lift)]
//! enum MyLittleOption<A> {
//!     Definitely(A),
//!     NotReally,
//! }
//! ```
//!
//! Now, to convert from `T<A>` to `T<B>`, you can get the `Target1` associated
//! type out of the `Lift` trait:
//!
//! ```nocompile
//! <MyLittleOption<A> as Lift<A, B>>::Target1
//! // this resolves to MyLittleOption<B>
//! ```
//!
//! There is also the `Lift3<A, B, C>` trait, which is also generated by the
//! `Lift` derive, which lets you go from `T<A>` to both `T<B>` and `T<C>` in
//! one go. This is useful if you need an intermediate type in one of your type
//! signatures, for instance a zip or merge function: `Fn(T<A>, T<B>) -> T<C>`.
//!
//! Here is how to use the above example type with `Lift3`:
//!
//! ```nocompile
//! <MyLittleOption<A> as Lift3<A, B, C>>::Target2
//! // this resolves to MyLittleOption<B>
//! <MyLittleOption<A> as Lift3<A, B, C>>::Target1
//! // this resolves to MyLittleOption<C>
//!
//! // the numbers go from right to left, so Target1 = C and Target2 = B.
//! ```
//!
//! Further, there's the `Bilift<A, B, C, D>` trait, for when you have two type
//! parameters to generalise over. This one takes you from `T<A, B>` to `T<C,
//! D>`. There's no corresponding `Bilift3` trait as yet, because I haven't
//! found a practical need for it yet.
//!
//! There's a corresponding derive for `Bilift`:
//!
//! ```nocompile
//! #[derive(Bilift)]
//! enum MyLittleResult<A, E> {
//!     Grand(A),
//!     NotGrand(E),
//! }
//! ```
//!
//! And, to get the derived type out of the `Bilift`:
//!
//! ```nocompile
//! <MyLittleResult<A, B> as Bilift<A, B, C, D>>::Target1
//! // this resolves to MyLittleResult<C, D>
//! ```
//!
//! # Yes, But Why, Really?
//!
//! Because sometimes one just gets homesick for Haskell and wants to implement
//! the `Functor` hierarchy. You'll find this in the `higher-cat` crate. It's
//! not really very suited for writing good Rust code, but it makes Haskell
//! programmers feel happy and it has a lot of funny words.
//!
//! [higher-derive]: https://docs.rs/crate/higher-derive

#![cfg_attr(not(feature = "std"), no_std)]

mod lift;
pub use crate::lift::{Lift, Lift3};

mod bilift;
pub use crate::bilift::Bilift;

/// You'd better be sure about what you're doing before using this.
pub(crate) fn unsafe_coerce<A, B>(mut a: A) -> B {
    unsafe {
        let ptr = &mut a as *mut _ as *mut B;
        let out = ::core::ptr::read(ptr);
        ::core::mem::forget(a);
        out
    }
}
