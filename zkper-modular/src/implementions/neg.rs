use super::*;
use std::ops::Neg;

impl<T: ZkperIntegerTrait, P: ZkperPrimeTrait<T>> Neg for ZkperModularInteger<T, P> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        P::negative(&self.value).into()
    }
}

impl<T: ZkperIntegerTrait, P: ZkperPrimeTrait<T>> Neg for &ZkperModularInteger<T, P> {
    type Output = ZkperModularInteger<T, P>;

    fn neg(self) -> Self::Output {
        P::negative(&self.value).into()
    }
}
