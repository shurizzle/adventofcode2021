use std::ops::{Add, AddAssign};

use num_traits::One;

pub trait Inc: Add<Self, Output = Self> + Sized {
    fn one() -> Self;

    fn inc(self) -> Self {
        Add::add(self, <Self as Inc>::one())
    }
}

impl<T: Add<Self, Output = Self> + One + Sized> Inc for T {
    fn one() -> Self {
        <Self as One>::one()
    }
}

pub trait IncAssign: Inc + AddAssign<Self> {
    fn inc_assign(&mut self) {
        AddAssign::add_assign(self, <Self as Inc>::one())
    }
}

impl<T: AddAssign<Self> + Inc> IncAssign for T {}
