use super::*;

impl<T: ZkperIntegerTrait> From<u64> for ZkperInteger<T> {
    fn from(u: u64) -> Self {
        Self(T::from_u64(u))
    }
}

impl<T: ZkperIntegerTrait> From<usize> for ZkperInteger<T> {
    fn from(u: usize) -> Self {
        Self(T::from_u64(u as u64))
    }
}

impl<T: ZkperIntegerTrait> From<i32> for ZkperInteger<T> {
    fn from(u: i32) -> Self {
        Self(T::from_i32(u))
    }
}

impl<T: ZkperIntegerTrait> From<u32> for ZkperInteger<T> {
    fn from(u: u32) -> Self {
        Self(T::from_u64(u as u64))
    }
}

impl<T: ZkperIntegerTrait> From<T> for ZkperInteger<T> {
    fn from(u: T) -> Self {
        Self(u)
    }
}

impl<T: ZkperIntegerTrait> From<&str> for ZkperInteger<T> {
    fn from(hex_str: &str) -> Self {
        Self(T::from_hex_str(hex_str))
    }
}
