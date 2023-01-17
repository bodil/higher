pub trait HeytingAlgebra {
    const TRUE: Self;
    const FALSE: Self;

    fn implies(self, other: Self) -> Self;
    fn conj(self, other: Self) -> Self;
    fn disj(self, other: Self) -> Self;
    fn not(self) -> Self;
}

impl HeytingAlgebra for bool {
    const TRUE: Self = true;

    const FALSE: Self = false;

    fn implies(self, other: Self) -> Self {
        !(self || other)
    }

    fn conj(self, other: Self) -> Self {
        self && other
    }

    fn disj(self, other: Self) -> Self {
        self || other
    }

    fn not(self) -> Self {
        !self
    }
}

impl HeytingAlgebra for () {
    const TRUE: Self = ();

    const FALSE: Self = ();

    fn implies(self, _other: Self) -> Self {
        ()
    }

    fn conj(self, _other: Self) -> Self {
        ()
    }

    fn disj(self, _other: Self) -> Self {
        ()
    }

    fn not(self) -> Self {
        ()
    }
}
