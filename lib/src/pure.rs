pub trait Pure<A> {
    /// Lift a value of type `A` into a `F<A>: Pure`.
    fn pure(value: A) -> Self;
}

impl<A> Pure<A> for Option<A> {
    fn pure(value: A) -> Self {
        Some(value)
    }
}

impl<A, E> Pure<A> for Result<A, E> {
    fn pure(value: A) -> Self {
        Ok(value)
    }
}

impl<A> Pure<A> for Vec<A> {
    fn pure(value: A) -> Self {
        vec![value]
    }
}
