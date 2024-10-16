use std::{cmp::Ordering, fmt::Debug, hash::Hash};

use zkper_rand::ZkperRng;

/// define behavior of zkper integer
pub trait ZkperIntegerTrait: Clone + Sized + Hash + Default + Debug {
    // generate integers
    fn from_i32(u: i32) -> Self;
    fn from_u32(u: u32) -> Self;
    fn from_u64(u: u64) -> Self;
    fn from_hex_str(hex_str: &str) -> Self;
    fn from_str(s: &str) -> Self;

    // compare
    // Comparison method
    fn compare(&self, other: &Self) -> Ordering;
    // Default implementations for PartialOrd
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
    // Default implementations for PartialEq
    fn eq(&self, other: &Self) -> bool {
        self.compare(other) == Ordering::Equal
    }

    // display
    fn to_string(&self) -> String;
    fn to_hex_string(&self) -> String;

    // basic values
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
    fn one() -> Self;
    fn is_one(&self) -> bool;
    fn two() -> Self;
    fn three() -> Self;
    fn four() -> Self;

    // bit operations
    fn to_bytes(&self) -> Vec<u8>;
    fn from_bytes(bytes: &[u8]) -> Self;
    // Returns the location of the first one, starting at start. If the bit at location start is one, returns start.
    fn find_first_one(&self, start: u32) -> Option<u32>;
    fn shr_32(&self, n: u32) -> Self;
    fn shr(&self, n: u64) -> Self;
    fn shl_32(&self, n: u32) -> Self;
    fn shl(&self, n: u64) -> Self;

    // basic operations
    fn sub(&self, rhs: &Self) -> Self;
    fn add(&self, rhs: &Self) -> Self;
    fn be_added(&self, rhs: Self) -> Self;
    fn add_u64(&self, rhs: u64) -> Self {
        self.add(&Self::from_u64(rhs))
    }
    fn neg(&self) -> Self;
    fn div(&self, rhs: &Self) -> Self;
    fn mul(&self, rhs: &Self) -> Self;
    fn be_muled(&self, rhs: Self) -> Self;
    fn square(&self) -> Self {
        self.mul(self)
    }
    fn rem(&self, rhs: &Self) -> Self;

    // extra for modular arithmetic
    fn pow_mod(&self, exp: &Self, modulus: &Self) -> Self;
    fn gcd(&self, other: &Self) -> Self;
    fn is_divisible(&self, other: &Self) -> bool {
        self.div(other).is_zero()
    }
    fn is_divisible_by_u64(&self, other: u64) -> bool {
        self.div(&Self::from_u64(other)).is_zero()
    }
    fn is_prime(&self) -> bool;
    fn invert(&self, modulus: &Self) -> anyhow::Result<Self>;

    // rand
    fn random_below(&self, rng: &mut ZkperRng) -> Self;

    // for signed integers
    fn abs(&self) -> Self {
        self.clone()
    }
}
