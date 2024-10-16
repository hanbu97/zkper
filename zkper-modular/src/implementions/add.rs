use super::*;
use std::ops::{Add, AddAssign};

impl<T: ZkperIntegerTrait, P: ZkperPrimeTrait<T>> Add for ZkperModularInteger<T, P> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        P::additive(&self.value, &rhs.value).into()
    }
}

impl<T: ZkperIntegerTrait, P: ZkperPrimeTrait<T>> Add<&ZkperModularInteger<T, P>>
    for ZkperModularInteger<T, P>
{
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        P::additive(&self.value, &rhs.value).into()
    }
}

impl<T: ZkperIntegerTrait, P: ZkperPrimeTrait<T>> AddAssign for ZkperModularInteger<T, P> {
    fn add_assign(&mut self, rhs: Self) {
        self.value = P::additive(&self.value, &rhs.value);
    }
}

impl<T: ZkperIntegerTrait, P: ZkperPrimeTrait<T>> AddAssign<&ZkperModularInteger<T, P>>
    for ZkperModularInteger<T, P>
{
    fn add_assign(&mut self, rhs: &Self) {
        self.value = P::additive(&self.value, &rhs.value);
    }
}
