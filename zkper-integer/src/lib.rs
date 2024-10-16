use traits::ZkperIntegerTrait;
use zkper_rand::ZkperRng;

pub mod backends;
pub mod implements;
pub mod traits;

// different integer backends
// some place to save integers from different backends
// later: will implement transfer between different backends.
//        e.g. from cpu to gpu
#[derive(Debug, Clone, Hash)]
pub struct ZkperInteger<T: ZkperIntegerTrait>(T);

impl<T: ZkperIntegerTrait> ZkperInteger<T> {
    pub fn from_i32(i: i32) -> Self {
        Self(T::from_i32(i))
    }

    pub fn from_str(s: &str) -> Self {
        Self(T::from_str(s))
    }

    pub fn new(integer: T) -> Self {
        Self(integer)
    }

    pub fn zero() -> Self {
        Self(T::zero())
    }

    pub fn one() -> Self {
        Self(T::one())
    }

    pub fn two() -> Self {
        Self(T::two())
    }

    pub fn three() -> Self {
        Self(T::three())
    }

    pub fn four() -> Self {
        Self(T::four())
    }

    pub fn subtract(&self, other: &Self) -> Self {
        Self(self.0.sub(&other.0))
    }

    pub fn additive(&self, other: &Self) -> Self {
        Self(self.0.add(&other.0))
    }

    pub fn be_added(&self, other: Self) -> Self {
        Self(self.0.be_added(other.0))
    }

    pub fn add_u64(&self, other: u64) -> Self {
        Self(self.0.add_u64(other))
    }

    pub fn neg(&self) -> Self {
        Self(self.0.neg())
    }

    pub fn divide(&self, other: &Self) -> Self {
        Self(self.0.div(&other.0))
    }

    pub fn multiply(&self, other: &Self) -> Self {
        Self(self.0.mul(&other.0))
    }

    pub fn be_muled(&self, other: Self) -> Self {
        Self(self.0.be_muled(other.0))
    }

    pub fn reminder(&self, other: &Self) -> Self {
        Self(self.0.rem(&other.0))
    }

    pub fn square(&self) -> Self {
        Self(self.0.square())
    }

    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    pub fn is_one(&self) -> bool {
        self.0.is_one()
    }

    pub fn is_not_one(&self) -> bool {
        !self.is_one()
    }

    pub fn from_hex_str(hex_str: &str) -> Self {
        Self(T::from_hex_str(hex_str))
    }

    pub fn pow_mod(&self, exp: &Self, modulus: &Self) -> Self {
        Self(self.0.pow_mod(&exp.0, &modulus.0))
    }

    pub fn is_divisible(&self, other: &Self) -> bool {
        self.0.is_divisible(&other.0)
    }

    pub fn is_prime(&self) -> bool {
        self.0.is_prime()
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    pub fn to_hex_string(&self) -> String {
        self.0.to_hex_string()
    }

    pub fn abs(&self) -> Self {
        Self(self.0.abs())
    }

    pub fn invert(&self, modulus: &Self) -> anyhow::Result<Self> {
        self.0.invert(&modulus.0).map(Self)
    }

    pub fn gcd(&self, other: &Self) -> Self {
        Self(self.0.gcd(&other.0))
    }

    pub fn random_below(&self, rng: &mut ZkperRng) -> Self {
        Self(self.0.random_below(rng))
    }

    pub fn find_first_one(&self, start: u32) -> Option<u32> {
        self.0.find_first_one(start)
    }

    pub fn shift_right_32(&self, n: u32) -> Self {
        Self(self.0.shr_32(n))
    }

    pub fn shift_right(&self, n: u64) -> Self {
        Self(self.0.shr(n))
    }

    pub fn shift_left_32(&self, n: u32) -> Self {
        Self(self.0.shl_32(n))
    }

    pub fn shift_left(&self, n: u64) -> Self {
        Self(self.0.shl(n))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes()
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self(T::from_bytes(bytes))
    }
}
