use traits::ZkperFieldTrait;
use zkper_integer::traits::ZkperIntegerTrait;
use zkper_modular::{traits::ZkperPrimeTrait, ZkperModularInteger};

pub mod backends;
pub mod traits;
pub mod utils;

#[derive(Debug, Clone)]
pub struct ZkperFieldElement<T: ZkperIntegerTrait, P: ZkperPrimeTrait<T>, F: ZkperFieldTrait<T, P>>
{
    pub value: ZkperModularInteger<T, P>,
    pub _field: std::marker::PhantomData<F>,
}
