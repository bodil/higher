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

## Licence

Copyright 2019 Bodil Stokke

This software is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/.

## Code of Conduct

Please note that this project is released with a [Contributor Code of Conduct][coc]. By
participating in this project you agree to abide by its terms.

[coc]: https://github.com/bodil/higher/blob/master/CODE_OF_CONDUCT.md
