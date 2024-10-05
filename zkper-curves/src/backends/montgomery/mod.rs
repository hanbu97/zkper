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

    /// (modulus + 1) / 4 if modulus % 4 == 3
    pub modulus_plus_one_div_four: Option<Integer>,

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

        let modulus_plus_one_div_four = if modulus.clone() % 4 != 3 {
            panic!("This sqrt implementation only works for p ≡ 3 (mod 4)");
        } else {
            Some((modulus.clone() + 1) / 4)
        };

        Self {
            modulus,
            r,
            r2,
            r3,
            inv,
            r_inv,
            modulus_plus_one_div_four,
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

    pub fn modulus_ref(&self) -> &Integer {
        &self.modulus
    }

    pub fn modulus(&self) -> Integer {
        self.modulus.clone()
    }

    pub fn r_ref(&self) -> &Integer {
        &self.r
    }

    pub fn r(&self) -> Integer {
        self.r.clone()
    }

    pub fn r_inv_ref(&self) -> &Integer {
        &self.r_inv
    }

    pub fn r_inv(&self) -> Integer {
        self.r_inv.clone()
    }

    pub fn r2_ref(&self) -> &Integer {
        &self.r2
    }

    pub fn r2(&self) -> Integer {
        self.r2.clone()
    }

    pub fn r3_ref(&self) -> &Integer {
        &self.r3
    }

    pub fn r3(&self) -> Integer {
        self.r3.clone()
    }

    pub fn inv_ref(&self) -> &Integer {
        &self.inv
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

    // Operations

    /// new element
    pub fn new_element(&self, value: Integer) -> Integer {
        // value % &self.modulus

        let modulus = &self.modulus;
        let normalized = value.clone() % modulus;
        if normalized < 0 {
            normalized + modulus
        } else {
            normalized
        }
    }

    /// Squares this element.
    pub fn square(&self, a: Integer) -> Integer {
        a.clone() * a % &self.modulus
    }

    /// Exponentiates this element by a given exponent.
    pub fn pow(&self, a: Integer, exp: &Integer) -> Integer {
        a.pow_mod(exp, &self.modulus).unwrap()
    }

    /// Negates this element.
    pub fn neg(&self, input: Integer) -> Integer {
        self.new_element(-input)
    }

    /// Attempts to compute the square root of this element.
    /// Shanks if p ≡ 3 (mod 4)
    pub fn sqrt(&self, input: Integer) -> Option<Integer> {
        match &self.modulus_plus_one_div_four {
            Some(exp) => {
                let sqrt = self.pow(input.clone(), exp);
                if self.square(sqrt.clone()) == input {
                    Some(self.new_element(sqrt))
                } else {
                    None
                }
            }
            None => panic!("This sqrt implementation only works for p ≡ 3 (mod 4)"),
        }
    }
}
