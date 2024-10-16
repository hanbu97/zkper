use super::*;
use std::ops::{Shr, ShrAssign};

impl<T: ZkperIntegerTrait> Shr<u32> for ZkperInteger<T> {
    type Output = Self;

    fn shr(self, rhs: u32) -> Self::Output {
        self.shift_right_32(rhs)
    }
}

impl<T: ZkperIntegerTrait> Shr<i32> for ZkperInteger<T> {
    type Output = Self;

    fn shr(self, rhs: i32) -> Self::Output {
        self.shift_right_32(rhs as u32)
    }
}

impl<T: ZkperIntegerTrait> Shr<u64> for ZkperInteger<T> {
    type Output = Self;

    fn shr(self, rhs: u64) -> Self::Output {
        self.shift_right(rhs)
    }
}

impl<T: ZkperIntegerTrait> Shr<usize> for ZkperInteger<T> {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        self.shift_right(rhs as u64)
    }
}

impl<T: ZkperIntegerTrait> ShrAssign<u32> for ZkperInteger<T> {
    fn shr_assign(&mut self, rhs: u32) {
        *self = self.shift_right_32(rhs);
    }
}

impl<T: ZkperIntegerTrait> ShrAssign<i32> for ZkperInteger<T> {
    fn shr_assign(&mut self, rhs: i32) {
        *self = self.shift_right_32(rhs as u32);
    }
}

impl<T: ZkperIntegerTrait> ShrAssign<u64> for ZkperInteger<T> {
    fn shr_assign(&mut self, rhs: u64) {
        *self = self.shift_right(rhs);
    }
}

impl<T: ZkperIntegerTrait> ShrAssign<usize> for ZkperInteger<T> {
    fn shr_assign(&mut self, rhs: usize) {
        *self = self.shift_right(rhs as u64);
    }
}
