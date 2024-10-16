use std::cmp::Ordering;

use super::*;

impl<T: ZkperIntegerTrait> PartialEq for ZkperInteger<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.compare(&other.0) == Ordering::Equal
    }
}

impl<T: ZkperIntegerTrait> Eq for ZkperInteger<T> {}

impl<T: ZkperIntegerTrait> PartialOrd for ZkperInteger<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.compare(&other.0))
    }
}

impl<T: ZkperIntegerTrait> Ord for ZkperInteger<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.compare(&other.0)
    }
}
