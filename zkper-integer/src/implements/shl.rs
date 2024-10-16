use super::*;
use std::ops::{Shl, ShlAssign};

impl<T: ZkperIntegerTrait> Shl<u32> for ZkperInteger<T> {
    type Output = Self;

    fn shl(self, rhs: u32) -> Self::Output {
        self.shift_left_32(rhs)
    }
}

impl<T: ZkperIntegerTrait> Shl<i32> for ZkperInteger<T> {
    type Output = Self;

    fn shl(self, rhs: i32) -> Self::Output {
        self.shift_left_32(rhs as u32)
    }
}

impl<T: ZkperIntegerTrait> Shl<u64> for ZkperInteger<T> {
    type Output = Self;

    fn shl(self, rhs: u64) -> Self::Output {
        self.shift_left(rhs)
    }
}

impl<T: ZkperIntegerTrait> Shl<usize> for ZkperInteger<T> {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        self.shift_left(rhs as u64)
    }
}

impl<T: ZkperIntegerTrait> ShlAssign<u32> for ZkperInteger<T> {
    fn shl_assign(&mut self, rhs: u32) {
        *self = self.shift_left_32(rhs);
    }
}

impl<T: ZkperIntegerTrait> ShlAssign<i32> for ZkperInteger<T> {
    fn shl_assign(&mut self, rhs: i32) {
        *self = self.shift_left_32(rhs as u32);
    }
}

impl<T: ZkperIntegerTrait> ShlAssign<u64> for ZkperInteger<T> {
    fn shl_assign(&mut self, rhs: u64) {
        *self = self.shift_left(rhs);
    }
}

impl<T: ZkperIntegerTrait> ShlAssign<usize> for ZkperInteger<T> {
    fn shl_assign(&mut self, rhs: usize) {
        *self = self.shift_left(rhs as u64);
    }
}
