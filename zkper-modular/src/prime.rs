use std::marker::PhantomData;

use zkper_integer::traits::ZkperIntegerTrait;

use crate::traits::ZkperPrimeTrait;

// #[derive(Debug, Clone, Hash)]
// pub struct ZkperPrime<T: ZkperIntegerTrait>(ZkperInteger<T>); // to store the prime number and define arithmetic operations

// impl<T: ZkperIntegerTrait> ZkperPrime<T> {
//     pub fn new(prime: ZkperInteger<T>) -> Self {
//         Self(prime)
//     }
// }

// Define ZkperPrime struct
#[derive(Debug, Clone, Hash)]
pub struct ZkperPrime<T: ZkperIntegerTrait, P: ZkperPrimeTrait<T>>(PhantomData<(T, P)>);
