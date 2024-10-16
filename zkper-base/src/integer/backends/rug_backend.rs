use crate::rand::ZkperRng;

use super::traits::ZkperIntegerTrait;
use num_traits::One;
use rand::RngCore;
use rug::{
    integer::{BorrowInteger, MiniInteger},
    rand::{RandGen, RandState, ThreadRandGen, ThreadRandState},
    Integer,
};
use std::str::FromStr;

#[derive(Debug, Clone, Hash, Default)]
pub struct RugBackend(pub Integer);

pub const INTEGER_TWO: &'static Integer = {
    const MINI: MiniInteger = MiniInteger::const_from_u8(2);
    const BORROW: BorrowInteger = MINI.borrow();
    BorrowInteger::const_deref(&BORROW)
};

pub const INTEGER_THREE: &'static Integer = {
    const MINI: MiniInteger = MiniInteger::const_from_u8(3);
    const BORROW: BorrowInteger = MINI.borrow();
    BorrowInteger::const_deref(&BORROW)
};

pub const INTEGER_FOUR: &'static Integer = {
    const MINI: MiniInteger = MiniInteger::const_from_u8(4);
    const BORROW: BorrowInteger = MINI.borrow();
    BorrowInteger::const_deref(&BORROW)
};

impl From<Integer> for RugBackend {
    fn from(value: Integer) -> Self {
        Self(value)
    }
}

impl RandGen for ZkperRng {
    fn gen(&mut self) -> u32 {
        self.next_u32()
    }

    fn gen_bits(&mut self, bits: u32) -> u32 {
        if bits == 0 {
            0
        } else if bits < 32 {
            self.next_u32() >> (32 - bits)
        } else {
            self.next_u32()
        }
    }

    fn seed(&mut self, seed: &Integer) {
        let seed_u64 = seed.to_u64_wrapping();
        *self = ZkperRng::from_seed(seed_u64);
    }
}

impl ThreadRandGen for ZkperRng {
    fn gen(&mut self) -> u32 {
        self.next_u32()
    }

    fn gen_bits(&mut self, bits: u32) -> u32 {
        if bits == 0 {
            0
        } else if bits < 32 {
            self.next_u32() >> (32 - bits)
        } else {
            self.next_u32()
        }
    }

    fn seed(&mut self, seed: &Integer) {
        let seed_u64 = seed.to_u64_wrapping();
        *self = ZkperRng::from_seed(seed_u64);
    }
}

// impl RugBackend {
//     pub fn rand() -> Self {
//         let t = Integer::from(100);
//         let mut zkper_rng = ZkperRng::new_test();
//         let mut rng: ThreadRandState<'_> = ThreadRandState::new_custom(&mut zkper_rng);

//         let t = t.random_below(&mut rng);

//         Self(0.into())
//     }
// }

impl ZkperIntegerTrait for RugBackend {
    fn from_hex_str(hex_str: &str) -> Self {
        let value =
            Integer::from_str_radix(hex_str.strip_prefix("0x").unwrap_or(hex_str), 16).unwrap();
        Self(value)
    }

    fn from_str(s: &str) -> Self {
        let value = Integer::from_str(s).unwrap();
        Self(value)
    }

    fn from_u64(u: u64) -> Self {
        let value = Integer::from(u);
        Self(value)
    }

    fn from_u32(u: u32) -> Self {
        let value = Integer::from(u);
        Self(value)
    }

    fn one() -> Self {
        Self(Integer::ONE.clone())
    }
    fn is_one(&self) -> bool {
        self.0.is_one()
    }

    fn zero() -> Self {
        Self(Integer::ZERO.clone())
    }
    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    fn two() -> Self {
        Self(INTEGER_TWO.clone())
    }

    fn three() -> Self {
        Self(INTEGER_THREE.clone())
    }

    fn four() -> Self {
        Self(INTEGER_FOUR.clone())
    }

    fn sub(&self, rhs: &Self) -> Self {
        Self(self.0.clone() - &rhs.0)
    }

    fn add(&self, rhs: &Self) -> Self {
        Self(self.0.clone() + &rhs.0)
    }

    fn add_u64(&self, rhs: u64) -> Self {
        Self(self.0.clone() + rhs)
    }

    fn neg(&self) -> Self {
        Self((-&self.0).into())
    }

    fn div(&self, rhs: &Self) -> Self {
        Self(self.0.clone() / &rhs.0)
    }

    fn mul(&self, rhs: &Self) -> Self {
        Self(self.0.clone() * &rhs.0)
    }

    fn square(&self) -> Self {
        Self(self.0.clone().square())
    }

    fn compare(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }

    fn pow_mod(&self, exp: &Self, modulus: &Self) -> Self {
        Self(self.0.clone().pow_mod(&exp.0, &modulus.0).unwrap())
    }

    fn is_divisible(&self, other: &Self) -> bool {
        self.0.is_divisible(&other.0)
    }

    fn is_prime(&self) -> bool {
        self.0.is_probably_prime(25) != rug::integer::IsPrime::No
    }

    fn invert(&self, modulus: &Self) -> anyhow::Result<Self> {
        self.0
            .clone()
            .invert(&modulus.0)
            .map(Self)
            .map_err(|e| anyhow::anyhow!(e))
    }

    fn to_string(&self) -> String {
        self.0.to_string()
    }

    fn to_hex_string(&self) -> String {
        self.0.to_string_radix(16)
    }

    fn abs(&self) -> Self {
        Self(self.0.clone().abs())
    }

    fn rem(&self, rhs: &Self) -> Self {
        Self(self.0.clone() % &rhs.0)
    }

    fn gcd(&self, other: &Self) -> Self {
        Self(self.0.clone().gcd(&other.0))
    }

    fn from_i32(u: i32) -> Self {
        Self(Integer::from(u))
    }

    fn random_below(&self, rng: &mut ZkperRng) -> Self {
        let mut rng: ThreadRandState<'_> = ThreadRandState::new_custom(rng);
        self.0.clone().random_below(&mut rng).into()
    }

    fn find_first_one(&self, start: u32) -> Option<u32> {
        self.0.find_one(start)
    }

    fn shr_32(&self, n: u32) -> Self {
        Self(self.0.clone() >> n)
    }

    // Note: 32-bit system
    fn shr(&self, n: u64) -> Self {
        Self(self.0.clone() >> (n as usize))
    }

    fn shl_32(&self, n: u32) -> Self {
        Self(self.0.clone() << n)
    }

    // Note: 32-bit system
    fn shl(&self, n: u64) -> Self {
        Self(self.0.clone() << (n as usize))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.0.to_digits(rug::integer::Order::Lsf)
    }
}

// fn add(&self, other: &Self) -> Self {
//     Self(self.0.clone() + &other.0)
// }

// fn mul(&self, other: &Self) -> Self {
//     Self(self.0.clone() * &other.0)
// }

// fn div(&self, other: &Self) -> Self {
//     Self(self.0.clone() / &other.0)
// }

// fn sqrt(&self) -> Self {
//     Self(self.0.clone().sqrt())
// }
