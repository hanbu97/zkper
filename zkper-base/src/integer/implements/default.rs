use super::*;

impl<T: ZkperIntegerTrait> Default for ZkperInteger<T> {
    fn default() -> Self {
        Self::zero()
    }
}
