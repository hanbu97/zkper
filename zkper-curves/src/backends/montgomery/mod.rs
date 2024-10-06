use rand_core::RngCore;
use rug::integer::BorrowInteger;
use rug::integer::MiniInteger;
use rug::Integer;
use std::ops::Mul;
use std::ops::Rem;

use crate::curves::bls12_381::Bls12_381BaseField;
use crate::traits::field::FieldTrait;

pub trait MontgomeryExt {
    fn from_montgomery_backend(&self, backend: &MontgomeryBackend) -> Integer;
}

impl MontgomeryExt for Integer {
    fn from_montgomery_backend(&self, backend: &MontgomeryBackend) -> Integer {
        backend.from_montgomery(self)
    }
}

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

pub const INTEGER_EIGHT: &'static Integer = {
    const MINI: MiniInteger = MiniInteger::const_from_u8(8);
    const BORROW: BorrowInteger = MINI.borrow();
    BorrowInteger::const_deref(&BORROW)
};

pub const INTEGER_TWELVE: &'static Integer = {
    const MINI: MiniInteger = MiniInteger::const_from_u8(12);
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
    /// montgomery form of 3b
    pub three_b_mont: Integer,

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
            None
        } else {
            Some((modulus.clone() + 1) / 4)
        };

        let three_b_mont = {
            let result = (INTEGER_TWELVE.clone() * &r2) % &modulus;
            (result * &r_inv) % &modulus
        };

        Self {
            modulus,
            r,
            r2,
            r3,
            inv,
            r_inv,
            modulus_plus_one_div_four,
            three_b_mont,
            limbs: limbs as usize,
        }
    }

    pub fn to_montgomery(&self, a: &Integer) -> Integer {
        self.mont_mul(a, &self.r2)
    }

    pub fn from_montgomery(&self, a: &Integer) -> Integer {
        self.mont_mul(a, Integer::ONE)
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

        let out = self.mont_mul(&d0, &self.r2) + self.mont_mul(&d1, &self.r3);

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

        let out = self.mont_mul(&d0, &self.r) + self.mont_mul(&d1, &self.r2);

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

    /// cubic
    pub fn cubic(&self, a: Integer) -> Integer {
        (a.clone() * &a % &self.modulus) * &a % &self.modulus
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

    /// Computes the multiplicative inverse of this element, if it exists.
    pub fn invert(&self, input: Integer) -> Option<Integer> {
        input
            .invert(&self.modulus)
            .map(|v| self.new_element(v))
            .ok()
    }

    /// Multiplies this element by another.
    pub fn mul(&self, input: Integer, other: &Integer) -> Integer {
        input * other % &self.modulus
    }

    /// add two elements
    pub fn add(&self, a: Integer, b: &Integer) -> Integer {
        (a + b) % &self.modulus
    }

    /// sub two elements
    pub fn sub(&self, a: Integer, b: &Integer) -> Integer {
        self.new_element(a - b)
    }

    // mont Operations

    /// montgomery multiplication
    pub fn mont_mul(&self, a: &Integer, b: &Integer) -> Integer {
        let result = (a.clone() * b) % &self.modulus;
        (result * &self.r_inv) % &self.modulus
    }

    pub fn mont_pow(&self, base: &Integer, exponent: &Integer) -> Integer {
        let mut result = self.r();
        let mut base = base.clone();
        let mut exp = exponent.clone();
        while !exp.is_zero() {
            if exp.is_odd() {
                result = self.mont_mul(&result, &base);
            }
            base = self.mont_mul(&base, &base);
            exp >>= 1;
        }
        result
    }

    /// point operations    

    /// Doubles a point (X, Y, Z) in standard projective coordinates.
    ///
    /// # Arguments
    ///
    /// * `x` - The X-coordinate of the point.
    /// * `y` - The Y-coordinate of the point.
    /// * `z` - The Z-coordinate of the point.
    ///
    /// # Returns
    ///
    /// A tuple `(X', Y', Z')` representing the doubled point in standard projective coordinates.
    ///
    /// This implementation assumes the curve parameter a = 0, which is true for both BLS12-381 and BLS12-377.
    /// These curves are defined by the equation y^2 = x^3 + b, where:
    /// - For BLS12-381: b = 4
    /// - For BLS12-377: b = 1
    ///
    /// The doubling formula used here is optimized for a = 0 curves.
    pub fn double_standard(
        &self,
        x: &Integer,
        y: &Integer,
        z: &Integer,
    ) -> (Integer, Integer, Integer) {
        // W = 3 * X^2 + a * Z^2, where a is the curve parameter (0 for BLS12-381)
        let x_sq = self.mul(x.clone(), x);
        let w = self.mul(x_sq.clone(), &INTEGER_THREE);

        // S = Y * Z
        let s = self.mul(y.clone(), z);

        // B = X * Y * S
        let xy = self.mul(x.clone(), y);
        let b = self.mul(xy.clone(), &s);

        // h = W^2 - 8 * B
        let w_sq = self.mul(w.clone(), &w);
        let eight_b = self.mul(b.clone(), &INTEGER_EIGHT);
        let h = self.sub(w_sq, &eight_b);

        // X' = 2 * h * S
        let two_h = self.mul(h.clone(), &INTEGER_TWO);
        let x_prime = self.mul(two_h, &s);

        // Y' = W * (4 * B - h) - 8 * Y^2 * S^2
        let four_b = self.mul(b, &INTEGER_FOUR);
        let four_b_minus_h = self.sub(four_b, &h);
        let w_term = self.mul(w, &four_b_minus_h);
        let y_sq = self.mul(y.clone(), y);
        let s_sq = self.mul(s.clone(), &s);
        let y_term = self.mul(y_sq, &s_sq);
        let eight_y_term = self.mul(y_term, &INTEGER_EIGHT);
        let y_prime = self.sub(w_term, &eight_y_term);

        // Z' = 8 * S^3
        let s_cube = self.mul(s_sq, &s);
        let z_prime = self.mul(s_cube, &INTEGER_EIGHT);

        (x_prime, y_prime, z_prime)
    }

    /// Doubles a point (X, Y, Z) using Montgomery arithmetic, following Algorithm 9.
    ///
    /// # Arguments
    ///
    /// * `x` - The X-coordinate of the point in Montgomery form.
    /// * `y` - The Y-coordinate of the point in Montgomery form.
    /// * `z` - The Z-coordinate of the point in Montgomery form.
    ///
    /// # Returns
    ///
    /// A tuple `(X', Y', Z')` representing the doubled point in Montgomery form.
    ///
    /// ref: https://eprint.iacr.org/2015/1060.pdf Algorithm 9
    pub fn double_mont(
        &self,
        x: &Integer,
        y: &Integer,
        z: &Integer,
    ) -> (Integer, Integer, Integer) {
        // 1. t0 ← Y · Y
        let mut t0 = self.mont_mul(y, y);

        // 2-3. Z3 ← t0 + t0, Z3 ← Z3 + Z3
        let mut z3 = self.add(t0.clone(), &t0);
        z3 = self.add(z3.clone(), &z3);

        // 4. Z3 ← Z3 + Z3
        z3 = self.add(z3.clone(), &z3);

        // 5. t1 ← Y · Z
        let t1 = self.mont_mul(y, z);

        // 6. t2 ← Z · Z
        let mut t2 = self.mont_mul(z, z);

        // 7. t2 ← b3 · t2 (b3 is 12(3*4) for bls12_381 · b, where b is the curve parameter)
        t2 = self.mont_mul(&t2, &self.three_b_mont);

        // 8. X3 ← t2 · Z3
        let mut x3 = self.mont_mul(&t2, &z3);

        // 9. Y3 ← t0 + t2
        let mut y3 = self.add(t0.clone(), &t2);

        // 10. Z3 ← t1 · Z3
        z3 = self.mont_mul(&t1, &z3);

        // 11. t1 ← t2 + t2
        let mut t1 = self.add(t2.clone(), &t2);

        // 12. t2 ← t1 + t2
        t2 = self.add(t1.clone(), &t2);

        // 13. t0 ← t0 - t2
        t0 = self.sub(t0, &t2);

        // 14. Y3 ← t0 · Y3
        y3 = self.mont_mul(&t0, &y3);

        // 15. Y3 ← X3 + Y3
        y3 = self.add(x3.clone(), &y3);

        // 16. t1 ← X · Y
        t1 = self.mont_mul(x, y);

        // 17. X3 ← t0 · t1
        x3 = self.mont_mul(&t0, &t1);

        // 18. X3 ← X3 + X3
        x3 = self.add(x3.clone(), &x3);

        (x3, y3, z3)
    }

    /// Adds two points (X1, Y1, Z1) and (X2, Y2, Z2) using Montgomery arithmetic, following Algorithm 7.
    ///
    /// # Arguments
    ///
    /// * `x1`, `y1`, `z1` - The coordinates of the first point in Montgomery form.
    /// * `x2`, `y2`, `z2` - The coordinates of the second point in Montgomery form.
    ///
    /// # Returns
    ///
    /// A tuple `(X3, Y3, Z3)` representing the sum of the two points in Montgomery form.
    /// ref: https://eprint.iacr.org/2015/1060.pdf Algorithm 7
    pub fn add_mont(
        &self,
        x1: &Integer,
        y1: &Integer,
        z1: &Integer,
        x2: &Integer,
        y2: &Integer,
        z2: &Integer,
    ) -> (Integer, Integer, Integer) {
        // 1. t0 ← X1 · X2
        let mut t0 = self.mont_mul(x1, x2);

        // 2. t1 ← Y1 · Y2
        let mut t1 = self.mont_mul(y1, y2);

        // 3. t2 ← Z1 · Z2
        let mut t2 = self.mont_mul(z1, z2);

        // 4. t3 ← X1 + Y1
        let mut t3 = self.add(x1.clone(), y1);

        // 5. t4 ← X2 + Y2
        let mut t4 = self.add(x2.clone(), y2);

        // 6. t3 ← t3 · t4
        t3 = self.mont_mul(&t3, &t4);

        // 7. t4 ← t0 + t1
        t4 = self.add(t0.clone(), &t1);

        // 8. t3 ← t3 - t4
        t3 = self.sub(t3, &t4);

        // 9. t4 ← Y1 + Z1
        t4 = self.add(y1.clone(), z1);

        // 10. X3 ← Y2 + Z2
        let mut x3 = self.add(y2.clone(), z2);

        // 11. t4 ← t4 · X3
        t4 = self.mont_mul(&t4, &x3);

        // 12. X3 ← t1 + t2
        x3 = self.add(t1.clone(), &t2);

        // 13. t4 ← t4 - X3
        t4 = self.sub(t4, &x3);

        // 14. X3 ← X1 + Z1
        x3 = self.add(x1.clone(), z1);

        // 15. Y3 ← X2 + Z2
        let mut y3 = self.add(x2.clone(), z2);

        // 16. X3 ← X3 · Y3
        x3 = self.mont_mul(&x3, &y3);

        // 17. Y3 ← t0 + t2
        y3 = self.add(t0.clone(), &t2);

        // 18. Y3 ← X3 - Y3
        y3 = self.sub(x3, &y3);

        // 19. X3 ← t0 + t0
        x3 = self.add(t0.clone(), &t0);

        // 20. t0 ← X3 + t0
        t0 = self.add(x3, &t0);

        // 21. t2 ← b3 · t2
        t2 = self.mont_mul(&self.three_b_mont, &t2);

        // 22. Z3 ← t1 + t2
        let mut z3 = self.add(t1.clone(), &t2);

        // 23. t1 ← t1 - t2
        t1 = self.sub(t1, &t2);

        // 24. Y3 ← b3 · Y3
        y3 = self.mont_mul(&self.three_b_mont, &y3);

        // 25. X3 ← t4 · Y3
        x3 = self.mont_mul(&t4, &y3);

        // 26. t2 ← t3 · t1
        t2 = self.mont_mul(&t3, &t1);

        // 27. X3 ← t2 - X3
        x3 = self.sub(t2, &x3);

        // 28. Y3 ← Y3 · t0
        y3 = self.mont_mul(&y3, &t0);

        // 29. t1 ← t1 · Z3
        t1 = self.mont_mul(&t1, &z3);

        // 30. Y3 ← t1 + Y3
        y3 = self.add(t1, &y3);

        // 31. t0 ← t0 · t3
        t0 = self.mont_mul(&t0, &t3);

        // 32. Z3 ← Z3 · t4
        z3 = self.mont_mul(&z3, &t4);

        // 33. Z3 ← Z3 + t0
        z3 = self.add(z3, &t0);

        (x3, y3, z3)
    }
}
