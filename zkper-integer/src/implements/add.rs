use super::*;
use std::ops::{Add, AddAssign};

// Implement Add for ZkperInteger<T>
impl<T: ZkperIntegerTrait> Add for ZkperInteger<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.additive(&rhs)
    }
}

// Implement Add<&ZkperInteger<T>> for ZkperInteger<T>
impl<T: ZkperIntegerTrait> Add<&ZkperInteger<T>> for ZkperInteger<T> {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        self.additive(rhs)
    }
}

// Implement Add<ZkperInteger<T>> for &ZkperInteger<T>
impl<T: ZkperIntegerTrait> Add<ZkperInteger<T>> for &ZkperInteger<T> {
    type Output = ZkperInteger<T>;

    fn add(self, rhs: ZkperInteger<T>) -> Self::Output {
        self.additive(&rhs)
    }
}

// Implement Add<&ZkperInteger<T>> for &ZkperInteger<T>
impl<T: ZkperIntegerTrait> Add<&ZkperInteger<T>> for &ZkperInteger<T> {
    type Output = ZkperInteger<T>;

    fn add(self, rhs: &ZkperInteger<T>) -> Self::Output {
        self.additive(rhs)
    }
}

// Implement AddAssign for ZkperInteger<T>
impl<T: ZkperIntegerTrait> AddAssign for ZkperInteger<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.additive(&rhs);
    }
}

// Implement AddAssign<&ZkperInteger<T>> for ZkperInteger<T>
impl<T: ZkperIntegerTrait> AddAssign<&ZkperInteger<T>> for ZkperInteger<T> {
    fn add_assign(&mut self, rhs: &Self) {
        *self = self.additive(rhs);
    }
}

// Optionally, implement Add and AddAssign for u64
impl<T: ZkperIntegerTrait> Add<u64> for ZkperInteger<T> {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        self.add_u64(rhs)
    }
}

impl<T: ZkperIntegerTrait> AddAssign<u64> for ZkperInteger<T> {
    fn add_assign(&mut self, rhs: u64) {
        *self = self.additive(&Self::from(rhs));
    }
}
