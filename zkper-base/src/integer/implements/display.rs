use super::*;
use std::fmt;

impl<T: ZkperIntegerTrait> fmt::Binary for ZkperInteger<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = self.to_bytes();
        for byte in bytes.iter() {
            write!(f, "{:08b}", byte)?;
        }
        Ok(())
    }
}

// Implement Display for ZkperInteger<T>
impl<T: ZkperIntegerTrait> fmt::Display for ZkperInteger<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

// Implement LowerHex for ZkperInteger<T>
impl<T: ZkperIntegerTrait> fmt::LowerHex for ZkperInteger<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex_string())
    }
}
