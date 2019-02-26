/// `Extract` lets you take a value of `A` out of an `F<A>`.
///
/// It is the categorical dual of `Pure`, ie. it does the opposite of `Pure`.
pub trait Extract<A> {
    fn extract(self) -> A;
}
