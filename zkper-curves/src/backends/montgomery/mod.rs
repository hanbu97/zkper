use num_traits::identities::One;
use num_traits::Pow;
use rand_core::RngCore;
use rug::integer::BorrowInteger;
use rug::integer::MiniInteger;
use rug::ops::DivRounding;
use rug::Integer;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Rem;
use std::ops::Sub;

use crate::curves::bls12_381::BLS12_381_BASE;

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

    /// for fp2 sqrt (modulus - 3) / 4 if modulus % 4 == 3. ref: https://eprint.iacr.org/2012/685.pdf algorithm 9
    pub fp2_sqrt_constant1: Option<Integer>,
    pub fp2_sqrt_constant2: Option<Integer>,

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

        let (modulus_plus_one_div_four, fp2_sqrt_constant1, fp2_sqrt_constant2) =
            if modulus.clone() % 4 != 3 {
                (None, None, None)
            } else {
                // (p + 1) / 4
                let c1 = (modulus.clone() + 1) / 4;

                let p = modulus.clone();
                let q: Integer = p ^ 2;

                // (q - 3) / 4
                let q1: Integer = ((q.clone() - 3) / 4) + 1;

                // (q - 1) / 2
                let q2: Integer = (q - 1) / 2 + 1;

                (Some(c1), Some(q1), Some(q2))
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
            fp2_sqrt_constant1,
            fp2_sqrt_constant2,
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
    pub fn sample_mont<R: RngCore>(&self, rng: &mut R) -> Integer {
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

    /// Squares this element in montegomery form.
    pub fn mont_square(&self, input: &Integer) -> Integer {
        self.mont_mul(input, input)
    }

    /// cubic
    pub fn cubic(&self, input: Integer) -> Integer {
        let square = self.square(input.clone());
        self.mul(square, &input)
    }

    /// cubic in montgomery form
    pub fn mont_cubic(&self, a: &Integer) -> Integer {
        let square = self.mont_square(a);
        self.mont_mul(&square, a)
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
    /// only fit for p ≡ 3 (mod 4) meet mont_sqrt req
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

    /// Shanks if p ≡ 3 (mod 4)

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
    /// Montgomery reduction: Computes (t * r^-1) mod n
    pub fn mont_reduction(&self, t: &Integer) -> Integer {
        let k = &self.inv; // Use precomputed inv
        let two64 = INTEGER_TWO.clone().pow(64);

        let m: Integer = (t.clone() * k) % &two64; // Faster modulo 2^64
        let mr = (m.clone() * &self.modulus) % &two64; // Faster modulo 2^64
        let u = (t.clone() + mr.clone() * &self.modulus) / two64;

        if u >= self.modulus {
            u - &self.modulus
        } else {
            u
        }
    }

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

    /// Computes the square root of a value in Montgomery form
    pub fn mont_sqrt(&self, input: &Integer) -> Option<Integer> {
        // The exponent (p+1)/4 in standard form
        match &self.modulus_plus_one_div_four {
            // why using original value?
            Some(exp) => {
                // Perform exponentiation in Montgomery form
                let sqrt = self.mont_pow(input, &exp);

                // Verify the result in Montgomery form
                let sqrt_squared = self.mont_mul(&sqrt, &sqrt);
                if &sqrt_squared == input {
                    Some(sqrt)
                } else {
                    None
                }
            }
            None => panic!("This sqrt implementation only works for p ≡ 3 (mod 4)"),
        }
    }

    /// point operations    
    ///
    ///// Normalizes projective coordinates to the form (X:Y:1)
    pub fn normalize(&self, x: &Integer, y: &Integer, z: &Integer) -> (Integer, Integer, Integer) {
        if z.is_one() {
            return (x.clone(), y.clone(), z.clone());
        }
        let z_inv = z
            .clone()
            .invert(self.modulus_ref())
            .expect("Z should be invertible");
        let x_norm = (x.clone() * &z_inv) % self.modulus_ref();
        let y_norm = (y.clone() * &z_inv) % self.modulus_ref();
        (x_norm, y_norm, Integer::from(1))
    }

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
    ///
    /// ref: https://leastauthority.com/static/publications/MoonMath080822.pdf Page 84 Algorithm 7 Projective short Weierstrass Addition Law
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

    /// Adds two points (X1, Y1, Z1) and (X2, Y2, Z2) using standard arithmetic, following Algorithm 7.
    ///
    /// # Arguments
    ///
    /// * `x1`, `y1`, `z1` - The coordinates of the first point in standard form.
    /// * `x2`, `y2`, `z2` - The coordinates of the second point in standard form.
    ///
    /// # Returns
    ///
    /// A tuple `(X3, Y3, Z3)` representing the sum of the two points in standard form.
    ///
    /// ref: https://leastauthority.com/static/publications/MoonMath080822.pdf Page 84 Algorithm 7 Projective short Weierstrass Addition Law

    pub fn add_standard(
        &self,
        x1: &Integer,
        y1: &Integer,
        z1: &Integer,
        x2: &Integer,
        y2: &Integer,
        z2: &Integer,
    ) -> (Integer, Integer, Integer) {
        // Check if first point is the point at infinity
        if z1.is_zero() {
            return (x2.clone(), y2.clone(), z2.clone());
        }

        // Check if second point is the point at infinity
        if z2.is_zero() {
            return (x1.clone(), y1.clone(), z1.clone());
        }

        // U1 ← Y2 · Z1
        let u1 = self.mul(y2.clone(), z1);

        // U2 ← Y1 · Z2
        let u2 = self.mul(y1.clone(), z2);

        // V1 ← X2 · Z1
        let v1 = self.mul(x2.clone(), z1);

        // V2 ← X1 · Z2
        let v2 = self.mul(x1.clone(), z2);

        if v1 == v2 {
            if u1 != u2 {
                // Points are inverses of each other, return point at infinity
                return (Integer::ZERO, Integer::ONE.clone(), Integer::ZERO);
            } else {
                if y1.is_zero() {
                    // Point is of order 2, return point at infinity
                    return (Integer::ZERO, Integer::ONE.clone(), Integer::ZERO);
                } else {
                    // Points are the same, use point doubling
                    return self.double_standard(x1, y1, z1);
                }
            }
        } else {
            // U = U1 - U2
            let u = self.sub(u1, &u2);

            // V = V1 - V2
            let v = self.sub(v1, &v2);

            // W = Z1 · Z2
            let w = self.mul(z1.clone(), z2);

            // A = U^2 · W - V^3 - 2 · V^2 · V2
            let a = self.sub(
                self.sub(self.mul(self.square(u.clone()), &w), &self.cubic(v.clone())),
                &self.mul(INTEGER_TWO.clone(), &self.mul(self.square(v.clone()), &v2)),
            );

            // X' = V · A
            let x3 = self.mul(v.clone(), &a);

            // Y' = U · (V^2 · V2 - A) - V^3 · U2
            let y3 = self.sub(
                self.mul(u, &self.sub(self.mul(self.square(v.clone()), &v2), &a)),
                &self.mul(self.cubic(v.clone()), &u2),
            );

            // Z' = V^3 · W
            let z3 = self.mul(self.cubic(v), &w);

            return (x3, y3, z3);
        }
    }
}

#[test]
fn test_weird_calculation() {
    let modulus = BLS12_381_BASE.modulus();
    println!("modulus: {}", modulus.to_string_radix(16));
}
