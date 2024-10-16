use std::marker::PhantomData;

use zkper_integer::traits::ZkperIntegerTrait;

/// type to define a finite field and its arithmetics
pub struct ZkperFiniteField<Integer: ZkperIntegerTrait> {
    pub modulus: Integer,
    pub integer: PhantomData<Integer>,
}

impl<Integer: ZkperIntegerTrait> ZkperFiniteField<Integer> {}
