pub trait Bilift<A, B, C, D> {
    type Target;
}

impl<A, B, C, D> Bilift<A, B, C, D> for Result<A, B> {
    type Target = Result<C, D>;
}
