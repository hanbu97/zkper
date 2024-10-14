use std::marker::PhantomData;

use crate::integer::traits::ZkperIntegerTrait;

/// type to define a finite field and its arithmetics
pub struct ZkperFiniteField<Integer: ZkperIntegerTrait> {
    pub modulus: Integer,
    pub integer: PhantomData<Integer>,
}

impl<Integer: ZkperIntegerTrait> ZkperFiniteField<Integer> {}
