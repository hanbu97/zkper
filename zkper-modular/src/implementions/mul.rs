use super::*;
use std::ops::{Mul, MulAssign};

impl<T: ZkperIntegerTrait, P: ZkperPrimeTrait<T>> Mul for ZkperModularInteger<T, P> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        P::multiply(&self.value, &rhs.value).into()
    }
}

impl<T: ZkperIntegerTrait, P: ZkperPrimeTrait<T>> Mul<&ZkperModularInteger<T, P>>
    for ZkperModularInteger<T, P>
{
    type Output = Self;

    fn mul(self, rhs: &Self) -> Self::Output {
        P::multiply(&self.value, &rhs.value).into()
    }
}

impl<T: ZkperIntegerTrait, P: ZkperPrimeTrait<T>> MulAssign for ZkperModularInteger<T, P> {
    fn mul_assign(&mut self, rhs: Self) {
        self.value = P::multiply(&self.value, &rhs.value);
    }
}

impl<T: ZkperIntegerTrait, P: ZkperPrimeTrait<T>> MulAssign<&ZkperModularInteger<T, P>>
    for ZkperModularInteger<T, P>
{
    fn mul_assign(&mut self, rhs: &Self) {
        self.value = P::multiply(&self.value, &rhs.value);
    }
}
