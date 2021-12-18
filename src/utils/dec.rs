use std::ops::{Sub, SubAssign};

use num_traits::One;

pub trait Dec: Sub<Self, Output = Self> + Sized {
    fn one() -> Self;

    fn dec(self) -> Self {
        Sub::sub(self, <Self as Dec>::one())
    }
}

impl<T: Sub<Self, Output = Self> + One + Sized> Dec for T {
    fn one() -> Self {
        <Self as One>::one()
    }
}

pub trait DecAssign: Dec + SubAssign<Self> {
    fn dec_assign(&mut self) {
        SubAssign::sub_assign(self, <Self as Dec>::one())
    }
}

impl<T: SubAssign<Self> + Dec> DecAssign for T {}
