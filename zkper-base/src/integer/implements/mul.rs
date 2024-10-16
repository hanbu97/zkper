use std::ops::{Mul, MulAssign};

use super::*;

// Implement Mul for ZkperInteger<T>
impl<T: ZkperIntegerTrait> Mul for ZkperInteger<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.multiply(&rhs)
    }
}

// Implement Mul<&ZkperInteger<T>> for ZkperInteger<T>
impl<T: ZkperIntegerTrait> Mul<&ZkperInteger<T>> for ZkperInteger<T> {
    type Output = Self;

    fn mul(self, rhs: &Self) -> Self::Output {
        self.multiply(rhs)
    }
}

// Implement Mul<ZkperInteger<T>> for &ZkperInteger<T>
impl<T: ZkperIntegerTrait> Mul<ZkperInteger<T>> for &ZkperInteger<T> {
    type Output = ZkperInteger<T>;

    fn mul(self, rhs: ZkperInteger<T>) -> Self::Output {
        self.multiply(&rhs)
    }
}

// Implement Mul<&ZkperInteger<T>> for &ZkperInteger<T>
impl<T: ZkperIntegerTrait> Mul<&ZkperInteger<T>> for &ZkperInteger<T> {
    type Output = ZkperInteger<T>;

    fn mul(self, rhs: &ZkperInteger<T>) -> Self::Output {
        self.multiply(rhs)
    }
}

// Implement MulAssign for ZkperInteger<T>
impl<T: ZkperIntegerTrait> MulAssign for ZkperInteger<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone().mul(&rhs);
    }
}

// Implement MulAssign<&ZkperInteger<T>> for ZkperInteger<T>
impl<T: ZkperIntegerTrait> MulAssign<&ZkperInteger<T>> for ZkperInteger<T> {
    fn mul_assign(&mut self, rhs: &Self) {
        *self = self.clone().mul(rhs);
    }
}

// Optionally, implement Mul and MulAssign for u64
impl<T: ZkperIntegerTrait> Mul<u64> for ZkperInteger<T> {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self::Output {
        self.mul(&Self::from(rhs))
    }
}

impl<T: ZkperIntegerTrait> MulAssign<u64> for ZkperInteger<T> {
    fn mul_assign(&mut self, rhs: u64) {
        *self = self.clone().mul(&Self::from(rhs));
    }
}

impl<T: ZkperIntegerTrait> MulAssign<usize> for ZkperInteger<T> {
    fn mul_assign(&mut self, rhs: usize) {
        *self = self.clone().mul(&Self::from(rhs));
    }
}
