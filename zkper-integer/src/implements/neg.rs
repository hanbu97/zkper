use std::ops::Neg;

use super::*;

impl<T: ZkperIntegerTrait> Neg for ZkperInteger<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(self.0.neg())
    }
}
