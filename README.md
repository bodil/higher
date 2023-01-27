# higher

The functor hierarchy and other terrible ideas for Rust.

Yes, this gives you generalisable monads in Rust. No, they're not very nice compared to Haskell,
because Rust's functions aren't quite as first class from the type system's perspective as you might
like them to be, type constraints in trait implementations can be a serious headache when you want
to implement, say, `Functor` for `HashSet`, and the type system can be particularly obtuse at times
and need a lot of additional and extremely verbose guidance to get the type inference right, but
they exist now.

What you get from this:

-   A set of fine grained traits (`Functor`, `Pure`, `Apply`, `Bind`, `Applicative` and `Monad`) for
    functors, applicatives and monads, inspired by
    [PureScript](https://pursuit.purescript.org/packages/purescript-prelude) and Scala's
    [Cats](https://typelevel.org/cats/).
-   Bifunctors, contravariant functors and profunctors, for completeness.
-   The `run!` macro for Haskell style do notation. I'd have preferred to call it `do!` or `for!`
    but unfortunately those are reserved keywords, even for macros.
-   Derive macros for `Functor` and `Bifunctor`.
-   Semigroups and monoids, because Rust's `Add` isn't quite a semigroup so `Add + Default` isn't
    quite a monoid.
-   Effect monads that wrap standard `Future`s and IO monads that wrap futures that can fail.
-   Most of `Foldable`, with the ambition of some of `Traversable` to follow. (It's always
    `traverse`.)
-   Rings and algebras, just in case.
-   Not necessarily a lot of good documentation, but like any good Haskell programmer you should be
    able to immediately infer every function's purpose from its type signature.

## What are your intentions with this?

I wrote this for two reasons: first, to see if it was even possible, and second, as a shitpost with
some extremely elaborate type signatures. If you think this is actually useful (and I'm mildly
horrified to find that I'm starting to think it might be), you may wish to step up to help maintain
it, because I doubt I'll keep giving it much attention once the novelty wears off.

## Is Rust actually capable of this?

To everyone's surprise, with
[generic associated types](https://blog.rust-lang.org/2022/10/28/gats-stabilization.html) we can now
express a subset of what higher kinded types give us in Rust. This subset turns out to be sufficient
to implement the abstractions found in the functor hierarchy, but the language still has its
limitations which have a significant impact on the usefulness of these abstractions.

### No constraint kinds

The first is the absence of constraint kinds. In Rust, when implementing a trait, we're unable to
require stricter constraint bounds in our implementations than the trait itself specifies. While
this has implications outside of GATs too, we run into it very quickly when trying to implement the
most basic part of the functor hierarchy: `Functor` itself.

We can express `Functor` as simply as possible like this:

```rust
trait Functor<A> {
    type Target<T>;
    fn fmap<B, F: Fn(A) -> B>(self, f: F) -> Self::Target<B>;
}
```

We can also implement it for the most basic types, such as `Vec`:

```rust
impl<A> Functor<A> for Vec<A> {
    type Target<T> = Vec<T>;
    fn fmap<B, F: Fn(A) -> B>(self, f: F) -> Self::Target<B> {
        self.into_iter().map(f).collect()
    }
}
```

But we immediately run into a problem when trying to implement it for a type with constraints on its
type arguments, like `HashSet`:

```rust
impl<A: Hash + Eq> Functor<A> for HashSet<A> {
    type Target<T> = HashSet<T>;
    fn fmap<B: Hash + Eq, F: Fn(A) -> B>(self, f: F) -> Self::Target<B> {
        self.into_iter().map(f).collect()
    }
}
```

Here, rustc will complain that the bounds on `B` are stricter than they are in the trait and refuse
to proceed, but we do need those bounds to be able to implement `fmap` for `HashSet`. There's no
trick available to us, even if we venture into nightly rustc's feature flags, to allow us to express
this properly, so we're left entirely unable to implement `Functor` for `HashSet`, or for anything
requiring bounds on `A`, as we have no way to express that they should carry over to the target
type.

There's an [open issue](https://github.com/rust-lang/rfcs/issues/2190) to address this situation
which has been going since 2017 without as much as a concrete proposal, so I don't think one should
allow oneself to feel too hopeful about a quick resolution to this problem.

### GATs aren't higher kinded types and using them as such gets messy

We all knew GATs wouldn't be quite as flexible as higher kinded types, but in some specific ways
they can be _more_ flexible, and not necessarily in a good way for our purposes. Let's illustrate
with an implementation of `Bind`:

```rust
trait Bind<A> {
    type Target<T>;
    fn then<B, F: Fn(A) -> Self::Target<B>>(self, f: F) -> Self::Target<B>;
}
```

If we'd like to write a function that composes two `Bind`s into a third, we'd expect it to look like
this:

```rust
fn compose_binds<A, B, C, M, F1, F2>(ma: M, f1: F1, f2: F2) -> M::Target<C>
where
    M: Bind<A>,
    F1: Fn(A) -> M::Target<B>,
    F2: Fn(B) -> M::Target<C>,
{
    ma.then(|a| f1(a).then(|b| f2(b)))
}
```

However, we have no guarantee that `M::Target<B>` actually resolves to anything that even implements
`Bind` in turn, even though we mean the `type Target<T>` in the trait to always refer back to the
original type constructor. This is just a convention on our part, a GAT in an of itself isn't
required to follow any such convention. We need to add trait bounds to clarify this in both cases:

```rust
fn compose_binds<A, B, C, M, F1, F2>(ma: M, f1: F1, f2: F2) -> M::Target<C>
where
    M: Bind<A>,
    M::Target<B>: Bind<B>,
    M::Target<C>: Bind<C>,
    F1: Fn(A) -> M::Target<B>,
    F2: Fn(B) -> M::Target<C>,
{
    ma.then(|a| f1(a).then(|b| f2(b)))
}
```

This is fair, and expected. However, we're not done yet, and this is where it gets hairy. We also
can't be sure that `<M::Target<B> as Thenable<B>>::Target<C>` will resolve to the same type as
`M::Target<C>`, so we have to modify our constraints to clarify that this should also be the case:

```rust
fn compose_binds<A, B, C, M, F1, F2>(ma: M, f1: F1, f2: F2) -> M::Target<C>
where
    M: Bind<A>,
    M::Target<B>: Bind<B, Target<C> = M::Target<C>>,
    M::Target<C>: Bind<C>,
    F1: Fn(A) -> M::Target<B>,
    F2: Fn(B) -> M::Target<C>,
{
    ma.then(|a| f1(a).then(|b| f2(b)))
}
```

In the above, one might think we could clarify it by specifying
`<M::Target<B> as Thenable<B>>::Target<C>` instead of `M::Target<C>` throughout, but this leads
nowhere better, as it turns out.

Finally, the type checker at this point fails to infer that the result type of the second bind call
should be `C`, so we need to explicitly provide it.

```rust
fn compose_binds<A, B, C, M, F1, F2>(ma: M, f1: F1, f2: F2) -> M::Target<C>
where
    M: Bind<A>,
    M::Target<B>: Bind<B, Target<C> = M::Target<C>>,
    M::Target<C>: Bind<C>,
    F1: Fn(A) -> M::Target<B>,
    F2: Fn(B) -> M::Target<C>,
{
    ma.then(|a| f1(a).then::<C, _>(|b| f2(b)))
}
```

This happens quite frequently, and is the reason the `run!` macro includes syntax for specifying the
type of a binding. Correct me if I'm wrong, but this part I can't excuse the type checker for, it
should have been able to figure this out for itself based on the function's return type.

In conclusion, GATs require considerably more effort to express everyday HTKs than you'd expect from
a language with HKTs, even though you _can_ express these things. This isn't a shortcoming of GATs
(in fact, it's a feature that HKTs don't provide), but it's a real problem when you're trying to
express abstractions like these which on the surface should have been much simpler. There's also the
added problem that the more you compensate for this behaviour, the more your implementation details
leak out into the type signature in ways they really shouldn't. This is especially unfortunate when
you're dealing with traits, where you'll need to predict ahead of time all the ways in which your
implementers will need this kind of extra support in the type signatures.

### Conclusion

So, is Rust ready for the functor hierarchy? We're frustratingly close, but I think in the end the
answer is no.

Basic abstractions like `Functor` would work well if we had constraint kinds, but as soon as you
step outside the basic use casees, you begin to see where GATs aren't an adequate substitute for
higher kinded types. That is to say, they solve a _different_ problem from HKTs, and it's not
straightforward to attempt to use them as a substitute. There are improvements which could be made
to Rust's type checker which could alleviate some, but not all or even most of these issues.

This entire crate should therefore be considered as an exercise, and perhaps as a guide towards new
language features, more than as a practical implementation. I do really see a genuine need for
constraint kinds in Rust beyond this crate. I also suspect Rust would have been better off with
actual higher kinded types, but that's a lost cause at this point.

Interestingly, though, while I was expecting Rust's borrow checker and the fact that it
differentiates so clearly between owned values and mutable and immutable references to be a primary
barrier to the implementation of this crate, this turned out to be more of an issue of API design
than anything else. The primary problem here was the inability to refine `Clone` requirements where
needed because of the absence of constraint kinds.

It's also unfortunate that I couldn't implement `Apply` for anything but boxed functions (as bare
functions with the same type signature on the surface aren't consistently typed in Rust, nor can
they be `Sized`), making it decidedly less than a zero cost abstraction, but I consider this a minor
issue even if it should make one cautious when describing Rust as a functional programming language
(which I don't think one should).

## Licence

Copyright 2019 Bodil Stokke

This software is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
was not distributed with this file, You can obtain one at <http://mozilla.org/MPL/2.0/>.

## Code of Conduct

Please note that this project is released with a [Contributor Code of Conduct][coc]. By
participating in this project you agree to abide by its terms.

[coc]: https://github.com/bodil/higher/blob/master/CODE_OF_CONDUCT.md
