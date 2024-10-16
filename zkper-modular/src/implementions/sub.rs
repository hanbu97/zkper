use std::ops::{Sub, SubAssign};

use super::*;

impl<T: ZkperIntegerTrait, P: ZkperPrimeTrait<T>> Sub for ZkperModularInteger<T, P> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        P::subtract(&self.value, &rhs.value).into()
    }
}

impl<T: ZkperIntegerTrait, P: ZkperPrimeTrait<T>> Sub<&ZkperModularInteger<T, P>>
    for ZkperModularInteger<T, P>
{
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        P::subtract(&self.value, &rhs.value).into()
    }
}

impl<T: ZkperIntegerTrait, P: ZkperPrimeTrait<T>> SubAssign for ZkperModularInteger<T, P> {
    fn sub_assign(&mut self, rhs: Self) {
        self.value = P::subtract(&self.value, &rhs.value);
    }
}

impl<T: ZkperIntegerTrait, P: ZkperPrimeTrait<T>> SubAssign<&ZkperModularInteger<T, P>>
    for ZkperModularInteger<T, P>
{
    fn sub_assign(&mut self, rhs: &Self) {
        self.value = P::subtract(&self.value, &rhs.value);
    }
}
