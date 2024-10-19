use rand::RngCore;

use super::*;

#[derive(Debug, Clone, Hash, Default)]
pub struct U32Backed(pub u32);

impl ZkperIntegerTrait for U32Backed {
    fn from_hex_str(hex_str: &str) -> Self {
        let value = u32::from_str_radix(hex_str.strip_prefix("0x").unwrap_or(hex_str), 16)
            .expect("Invalid hex string");

        Self(value)
    }

    fn from_str(s: &str) -> Self {
        let value = u32::from_str(s).expect("Invalid string");
        Self(value)
    }

    fn from_u64(u: u64) -> Self {
        Self(u as u32)
    }

    fn from_u32(u: u32) -> Self {
        Self(u)
    }

    fn one() -> Self {
        Self(1)
    }

    fn is_one(&self) -> bool {
        self.0 == 1
    }

    fn zero() -> Self {
        Self(0)
    }

    fn is_zero(&self) -> bool {
        self.0 == 0
    }

    fn two() -> Self {
        Self(2)
    }

    fn three() -> Self {
        Self(3)
    }

    fn four() -> Self {
        Self(4)
    }

    fn sub(&self, rhs: &Self) -> Self {
        Self(self.0.wrapping_sub(rhs.0))
    }

    fn add(&self, rhs: &Self) -> Self {
        Self(self.0.wrapping_add(rhs.0))
    }

    fn be_added(&self, rhs: Self) -> Self {
        Self(rhs.0.wrapping_add(self.0))
    }

    fn add_u64(&self, rhs: u64) -> Self {
        Self(self.0.wrapping_add(rhs as u32))
    }

    fn neg(&self) -> Self {
        Self(self.0.wrapping_neg())
    }

    fn div(&self, rhs: &Self) -> Self {
        Self(self.0.wrapping_div(rhs.0))
    }

    fn mul(&self, rhs: &Self) -> Self {
        Self(self.0.wrapping_mul(rhs.0))
    }

    fn be_muled(&self, rhs: Self) -> Self {
        Self(rhs.0.wrapping_mul(self.0))
    }

    fn square(&self) -> Self {
        Self(self.0.wrapping_mul(self.0))
    }

    fn compare(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }

    fn pow_mod(&self, exp: &Self, modulus: &Self) -> Self {
        Self(self.0.wrapping_pow(exp.0) % modulus.0)
    }

    fn is_divisible(&self, other: &Self) -> bool {
        self.0 % other.0 == 0
    }

    fn is_prime(&self) -> bool {
        if self.0 <= 1 {
            return false;
        }
        for i in 2..=(self.0 as f64).sqrt() as u32 {
            if self.0 % i == 0 {
                return false;
            }
        }
        true
    }

    fn invert(&self, modulus: &Self) -> anyhow::Result<Self> {
        for i in 1..modulus.0 {
            if (self.0 as u64 * i as u64) % modulus.0 as u64 == 1 {
                return Ok(Self(i));
            }
        }
        Err(anyhow::anyhow!("No modular inverse found"))
    }

    fn to_string(&self) -> String {
        self.0.to_string()
    }

    fn to_hex_string(&self) -> String {
        format!("{:x}", self.0)
    }

    fn abs(&self) -> Self {
        Self(self.0)
    }

    fn rem(&self, rhs: &Self) -> Self {
        Self(self.0 % rhs.0)
    }

    fn gcd(&self, other: &Self) -> Self {
        let mut a = self.0;
        let mut b = other.0;
        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }
        Self(a)
    }

    fn from_i32(u: i32) -> Self {
        Self(u as u32)
    }

    fn random_below(&self, rng: &mut ZkperRng) -> Self {
        Self(rng.next_u32() % self.0)
    }

    fn find_first_one(&self, start: u32) -> Option<u32> {
        for i in start..32 {
            if (self.0 & (1 << i)) != 0 {
                return Some(i);
            }
        }
        None
    }

    fn shr_32(&self, n: u32) -> Self {
        Self(self.0 >> n)
    }

    fn shr(&self, n: u64) -> Self {
        Self(self.0 >> n)
    }

    fn shl_32(&self, n: u32) -> Self {
        Self(self.0 << n)
    }

    fn shl(&self, n: u64) -> Self {
        Self(self.0 << n)
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.0.to_le_bytes().to_vec()
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        let mut array = [0u8; 4];
        array.copy_from_slice(&bytes[..4]);
        Self(u32::from_le_bytes(array))
    }

    fn is_even(&self) -> bool {
        self.0 % 2 == 0
    }

    fn is_odd(&self) -> bool {
        self.0 % 2 != 0
    }
}
