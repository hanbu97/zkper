use std::{marker::PhantomData, ops::Add};

use traits::ZkperPrimeTrait;
use zkper_integer::{traits::ZkperIntegerTrait, ZkperInteger};

pub mod backends;
pub mod implementions;
pub mod prime;
pub mod traits;

pub struct ZkperModularInteger<T: ZkperIntegerTrait, P: ZkperPrimeTrait<T>> {
    pub value: ZkperInteger<T>,
    _prime: PhantomData<P>,
}

impl<T: ZkperIntegerTrait, P: ZkperPrimeTrait<T>> From<ZkperInteger<T>>
    for ZkperModularInteger<T, P>
{
    fn from(value: ZkperInteger<T>) -> Self {
        Self::new(value)
    }
}

impl<T: ZkperIntegerTrait, P: ZkperPrimeTrait<T>> ZkperModularInteger<T, P> {
    pub fn new(value: ZkperInteger<T>) -> Self {
        Self {
            value: value,
            _prime: PhantomData,
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use zkper_integer::{
//         backends::rug_backend::RugBackend, traits::ZkperIntegerTrait, ZkperInteger,
//     };

//     use crate::{traits::ZkperPrimeTrait, ZkperModularInteger};

//     struct Prime17;

//     impl<T: ZkperIntegerTrait> ZkperPrimeTrait<T> for Prime17 {
//         fn value() -> ZkperInteger<T> {
//             ZkperInteger::from(17)
//         }
//     }

//     #[test]
//     fn test_zkper_modular_integer() {
//         let modular_int = ZkperModularInteger::<RugBackend, Prime17>::new(ZkperInteger::from(10));
//     }
// }
