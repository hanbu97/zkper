use rand_core::RngCore;
use rug::integer::BorrowInteger;
use rug::integer::MiniInteger;
use rug::Integer;
use std::ops::Mul;
use std::ops::Rem;

pub const INTEGER_TWO: &'static Integer = {
    const MINI: MiniInteger = MiniInteger::const_from_u8(2);
    const BORROW: BorrowInteger = MINI.borrow();
    BorrowInteger::const_deref(&BORROW)
};

pub struct MontgomeryBackend {
    /// The modulus of the field.
    pub modulus: Integer,
    /// R = 2^(NUM_LIMBS*64) mod MODULUS
    pub r: Integer, // one
    /// R2 = R^2 mod MODULUS
    pub r2: Integer,

    /// R_INV = R^(-1) mod MODULUS
    pub r_inv: Integer,
    /// R3 = R^3 mod MODULUS
    pub r3: Integer,

    /// INV = -MODULUS^{-1} mod 2^64
    pub inv: Integer,

    /// limbs
    pub limbs: usize,
}

impl MontgomeryBackend {
    pub fn new(modulus: Integer, limbs: u64) -> Self {
        let r = Self::compute_r(&modulus, limbs);
        let r2 = r
            .clone()
            .pow_mod(INTEGER_TWO, &modulus)
            .expect("R^2 should be computed");
        let r3 = r.clone().mul(&r2).rem(&modulus);
        let inv = Self::compute_inv(&modulus);
        let r_inv = r.clone().invert(&modulus).expect("R should be invertible");

        Self {
            modulus,
            r,
            r2,
            r3,
            inv,
            r_inv,
            limbs: limbs as usize,
        }
    }

    pub fn to_montgomery(&self, a: &Integer) -> Integer {
        self.montgomery_multiply(a, &self.r2)
    }

    pub fn from_montgomery(&self, a: &Integer) -> Integer {
        self.montgomery_multiply(a, Integer::ONE)
    }

    pub fn montgomery_multiply(&self, a: &Integer, b: &Integer) -> Integer {
        let result = (a.clone() * b) % &self.modulus;
        (result * &self.r_inv) % &self.modulus
    }

    pub fn modulus(&self) -> Integer {
        self.modulus.clone()
    }

    pub fn r(&self) -> Integer {
        self.r.clone()
    }

    pub fn r_inv(&self) -> Integer {
        self.r_inv.clone()
    }

    pub fn r2(&self) -> Integer {
        self.r2.clone()
    }

    pub fn inv(&self) -> Integer {
        self.inv.clone()
    }

    pub fn limbs(&self) -> usize {
        self.limbs
    }

    pub fn from_str_radix(s: &str, radix: i32, limbs: u64) -> Self {
        let modulus = Integer::from_str_radix(s, radix).expect("invalid modulus");
        Self::new(modulus, limbs)
    }

    /// Compute R = 2^(NUM_LIMBS*64) mod MODULUS
    fn compute_r(modulus: &Integer, limbs: u64) -> Integer {
        INTEGER_TWO
            .clone()
            .pow_mod(&Integer::from(limbs * 64), modulus)
            .expect("R should be computed")
    }

    /// Compute -M^{-1} mod 2^64
    fn compute_inv(modulus: &Integer) -> Integer {
        let modulus_64 = modulus.to_u64_wrapping();
        let mut inv = 1u64;
        for _ in 0..63 {
            inv = inv.wrapping_mul(inv);
            inv = inv.wrapping_mul(modulus_64);
        }
        inv.wrapping_neg().into()
    }

    /// Sample a random value in montgomery form
    pub fn sample<R: RngCore>(&self, rng: &mut R) -> Integer {
        let bytes_needed = self.limbs * 16;
        let mut bytes = vec![0u8; bytes_needed];
        rng.fill_bytes(&mut bytes);

        let order = if self.limbs == 4 {
            rug::integer::Order::Lsf
        } else {
            rug::integer::Order::Msf
        };
        let d0 = Integer::from_digits(&bytes[..bytes_needed / 2], order);
        let d1 = Integer::from_digits(&bytes[bytes_needed / 2..], order);

        let out = self.montgomery_multiply(&d0, &self.r2) + self.montgomery_multiply(&d1, &self.r3);

        out % &self.modulus
    }

    /// sample a raw (non-Montgomery) value
    pub fn sample_raw<R: RngCore>(&self, rng: &mut R) -> Integer {
        let bytes_needed = self.limbs * 16;
        let mut bytes = vec![0u8; bytes_needed];
        rng.fill_bytes(&mut bytes);

        let order = if self.limbs == 4 {
            rug::integer::Order::Lsf
        } else {
            rug::integer::Order::Msf
        };
        let d0 = Integer::from_digits(&bytes[..bytes_needed / 2], order);
        let d1 = Integer::from_digits(&bytes[bytes_needed / 2..], order);

        let out = self.montgomery_multiply(&d0, &self.r) + self.montgomery_multiply(&d1, &self.r2);

        out % &self.modulus
    }
}
