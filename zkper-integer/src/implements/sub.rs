use super::*;
use std::ops::{Sub, SubAssign};

// Implement Sub for ZkperInteger<T>
impl<T: ZkperIntegerTrait> Sub for ZkperInteger<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.subtract(&rhs)
    }
}

// Implement Sub<&ZkperInteger<T>> for ZkperInteger<T>
impl<T: ZkperIntegerTrait> Sub<&ZkperInteger<T>> for ZkperInteger<T> {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        self.subtract(rhs)
    }
}

// Implement Sub<ZkperInteger<T>> for &ZkperInteger<T>
impl<T: ZkperIntegerTrait> Sub<ZkperInteger<T>> for &ZkperInteger<T> {
    type Output = ZkperInteger<T>;

    fn sub(self, rhs: ZkperInteger<T>) -> Self::Output {
        self.subtract(&rhs)
    }
}

// Implement Sub<&ZkperInteger<T>> for &ZkperInteger<T>
impl<T: ZkperIntegerTrait> Sub<&ZkperInteger<T>> for &ZkperInteger<T> {
    type Output = ZkperInteger<T>;

    fn sub(self, rhs: &ZkperInteger<T>) -> Self::Output {
        self.subtract(rhs)
    }
}

// Implement SubAssign for ZkperInteger<T>
impl<T: ZkperIntegerTrait> SubAssign for ZkperInteger<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.subtract(&rhs);
    }
}

// Implement SubAssign<&ZkperInteger<T>> for ZkperInteger<T>
impl<T: ZkperIntegerTrait> SubAssign<&ZkperInteger<T>> for ZkperInteger<T> {
    fn sub_assign(&mut self, rhs: &Self) {
        *self = self.subtract(rhs);
    }
}

// Optionally, implement Sub and SubAssign for u64
impl<T: ZkperIntegerTrait> Sub<u64> for ZkperInteger<T> {
    type Output = Self;

    fn sub(self, rhs: u64) -> Self::Output {
        self.subtract(&Self::from(rhs))
    }
}

impl<T: ZkperIntegerTrait> SubAssign<u64> for ZkperInteger<T> {
    fn sub_assign(&mut self, rhs: u64) {
        *self = self.subtract(&Self::from(rhs));
    }
}

impl<T: ZkperIntegerTrait> Sub<u64> for &ZkperInteger<T> {
    type Output = ZkperInteger<T>;

    fn sub(self, rhs: u64) -> Self::Output {
        ZkperInteger(self.0.sub(&T::from_u64(rhs)))
    }
}
