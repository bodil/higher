window.SIDEBAR_ITEMS = {"fn":[["ap","`ap` is a default implementation of [`Apply::apply`][Apply::apply] for any type that implements `Bind`, `Pure` and `Clone`."],["apply_first",""],["apply_second",""],["lift2",""]],"struct":[["ApplyFn","An `ApplyFn` is a function from `A` to `B` wrapped in something Rust’s type system can more easily digest. Arguments for `Apply::apply()` are required to be of this type rather than an arbitrary type matching `Fn(A) -> B`."]],"trait":[["Apply","`Apply` takes an `F<Fn(A) -> B>` (or, rather, an `F<ApplyFn<'a,A, B>>` specifically) and applies it to an `F<A>` to produce an `F<B>`."]]};