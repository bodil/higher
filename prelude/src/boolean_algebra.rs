use crate::HeytingAlgebra;

pub trait BooleanAlgebra: HeytingAlgebra {}

impl BooleanAlgebra for bool {}
impl BooleanAlgebra for () {}
