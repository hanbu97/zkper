use std::ops::Rem;

use super::*;

// Implement Rem trait for ZkperInteger
impl<T: ZkperIntegerTrait> std::ops::Rem for ZkperInteger<T> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        self.reminder(&rhs)
    }
}

// &ZkperInteger % ZkperInteger
impl<T: ZkperIntegerTrait> Rem<ZkperInteger<T>> for &ZkperInteger<T> {
    type Output = ZkperInteger<T>;

    fn rem(self, rhs: ZkperInteger<T>) -> Self::Output {
        self.reminder(&rhs)
    }
}

// &ZkperInteger % &ZkperInteger
impl<T: ZkperIntegerTrait> Rem<&ZkperInteger<T>> for &ZkperInteger<T> {
    type Output = ZkperInteger<T>;

    fn rem(self, rhs: &ZkperInteger<T>) -> Self::Output {
        self.reminder(rhs)
    }
}

impl<T: ZkperIntegerTrait> std::ops::Rem<&ZkperInteger<T>> for ZkperInteger<T> {
    type Output = ZkperInteger<T>;

    fn rem(self, rhs: &ZkperInteger<T>) -> Self::Output {
        self.reminder(rhs)
    }
}

// Implement RemAssign trait for ZkperInteger
impl<T: ZkperIntegerTrait> std::ops::RemAssign for ZkperInteger<T> {
    fn rem_assign(&mut self, rhs: Self) {
        *self = self.reminder(&rhs);
    }
}

impl<T: ZkperIntegerTrait> std::ops::RemAssign<&ZkperInteger<T>> for ZkperInteger<T> {
    fn rem_assign(&mut self, rhs: &ZkperInteger<T>) {
        *self = self.reminder(rhs);
    }
}
