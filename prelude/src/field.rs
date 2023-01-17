use crate::{DivisionRing, EuclideanRing};

pub trait Field: EuclideanRing + DivisionRing {}

impl<A> Field for A where A: EuclideanRing + DivisionRing {}
