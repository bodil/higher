use crate::Ring;

pub trait DivisionRing: Ring {
    fn recip(self) -> Self;

    fn left_div(self, other: Self) -> Self
    where
        Self: Sized,
    {
        other.mul(self).recip()
    }

    fn right_div(self, other: Self) -> Self
    where
        Self: Sized,
    {
        self.mul(other.recip())
    }
}

impl DivisionRing for f32 {
    fn recip(self) -> Self {
        1.0 / self
    }
}

impl DivisionRing for f64 {
    fn recip(self) -> Self {
        1.0 / self
    }
}
