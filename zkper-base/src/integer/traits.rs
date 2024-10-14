/// define behavior of zkper integer
pub trait ZkperIntegerTrait {
    // generate integers
    fn from_u64(u: u64) -> Self;
    fn from_hex_str(hex_str: &str) -> Self;

    // basic values
    fn zero() -> Self;
    fn one() -> Self;
    fn two() -> Self;

    // basic operations
    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
    fn div(&self, other: &Self) -> Self;
    fn neg(&self) -> Self;
    fn sqrt(&self) -> Self;
}
