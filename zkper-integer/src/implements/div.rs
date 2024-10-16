use std::ops::{Div, DivAssign};

use super::*;

// Implement Div for ZkperInteger<T>
impl<T: ZkperIntegerTrait> Div for ZkperInteger<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0.div(&rhs.0))
    }
}

// Implement Div<&ZkperInteger<T>> for ZkperInteger<T>
impl<T: ZkperIntegerTrait> Div<&ZkperInteger<T>> for ZkperInteger<T> {
    type Output = Self;

    fn div(self, rhs: &Self) -> Self::Output {
        Self(self.0.div(&rhs.0))
    }
}

// Implement Div<ZkperInteger<T>> for &ZkperInteger<T>
impl<T: ZkperIntegerTrait> Div<ZkperInteger<T>> for &ZkperInteger<T> {
    type Output = ZkperInteger<T>;

    fn div(self, rhs: ZkperInteger<T>) -> Self::Output {
        ZkperInteger(self.0.div(&rhs.0))
    }
}

// Implement Div<&ZkperInteger<T>> for &ZkperInteger<T>
impl<T: ZkperIntegerTrait> Div<&ZkperInteger<T>> for &ZkperInteger<T> {
    type Output = ZkperInteger<T>;

    fn div(self, rhs: &ZkperInteger<T>) -> Self::Output {
        ZkperInteger(self.0.div(&rhs.0))
    }
}

// Implement DivAssign for ZkperInteger<T>
impl<T: ZkperIntegerTrait> DivAssign for ZkperInteger<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.0 = self.0.div(&rhs.0);
    }
}

// Implement DivAssign<&ZkperInteger<T>> for ZkperInteger<T>
impl<T: ZkperIntegerTrait> DivAssign<&ZkperInteger<T>> for ZkperInteger<T> {
    fn div_assign(&mut self, rhs: &Self) {
        self.0 = self.0.div(&rhs.0);
    }
}

// Optionally, implement Div and DivAssign for u64
impl<T: ZkperIntegerTrait> Div<u64> for ZkperInteger<T> {
    type Output = Self;

    fn div(self, rhs: u64) -> Self::Output {
        Self(self.0.div(&T::from_u64(rhs)))
    }
}

impl<T: ZkperIntegerTrait> DivAssign<u64> for ZkperInteger<T> {
    fn div_assign(&mut self, rhs: u64) {
        self.0 = self.0.div(&T::from_u64(rhs));
    }
}
