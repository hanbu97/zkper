use std::{fmt::Debug, hash::Hash};

use zkper_integer::traits::ZkperIntegerTrait;
use zkper_modular::traits::ZkperPrimeTrait;

/// define behavior of a finite field
pub trait ZkperFieldTrait<T: ZkperIntegerTrait, P: ZkperPrimeTrait<T>>:
    Clone + Sized + Hash + Default + Debug
{
}
