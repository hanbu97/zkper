use crate::{curves::bls12_381::BLS12_381_BASE, traits::field::FieldTrait};
use rug::Integer;
use std::{fmt::Display, str::FromStr};

use super::base::Bls12_381BaseField;
use num_traits::One;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fp2 {
    pub c0: Integer, // Base field element c0
    pub c1: Integer, // Base field element c1
}

impl Display for Fp2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(\n    {} +\n    {} * u\n)",
            self.c0.to_string_radix(16),
            self.c1.to_string_radix(16)
        )
    }
}

impl Fp2 {
    pub fn from_strs(c0: &str, c1: &str) -> Self {
        let c0 = Integer::from_str(c0).unwrap();
        let c1 = Integer::from_str(c1).unwrap();

        Self { c0, c1 }
    }

    pub fn from_hexs(c0: &str, c1: &str) -> Self {
        let c0 = Integer::from_str_radix(c0.strip_prefix("0x").unwrap_or(c0), 16).unwrap();
        let c1 = Integer::from_str_radix(c1.strip_prefix("0x").unwrap_or(c1), 16).unwrap();

        Self { c0, c1 }
    }

    pub fn from_integers(c0: Integer, c1: Integer) -> Self {
        Self { c0, c1 }
    }

    pub fn from_u64_hex_str_vec(c0: &[&str], c1: &[&str]) -> Self {
        let c0 = Bls12_381BaseField::from_u64_hex_str_vec(c0);
        let c1 = Bls12_381BaseField::from_u64_hex_str_vec(c1);

        Self { c0, c1 }
    }

    pub fn from_u64_vec(c0: &[u64], c1: &[u64]) -> Self {
        let c0 = Bls12_381BaseField::from_u64_vec(c0);
        let c1 = Bls12_381BaseField::from_u64_vec(c1);

        Self { c0, c1 }
    }

    pub fn from_mont(&self) -> Self {
        let c0 = BLS12_381_BASE.from_montgomery(&self.c0);
        let c1 = BLS12_381_BASE.from_montgomery(&self.c1);

        Self { c0, c1 }
    }

    pub fn to_mont(&self) -> Self {
        let c0 = BLS12_381_BASE.to_montgomery(&self.c0);
        let c1 = BLS12_381_BASE.to_montgomery(&self.c1);

        Self { c0, c1 }
    }

    pub fn zero() -> Self {
        Self {
            c0: Integer::from(0),
            c1: Integer::from(0),
        }
    }

    pub fn one() -> Self {
        Self {
            c0: Bls12_381BaseField::one(),
            c1: Integer::from(0),
        }
    }

    pub fn is_one(&self) -> bool {
        self.c0.is_one() && self.c1.is_zero()
    }

    pub fn is_zero(&self) -> bool {
        self.c0.is_zero() && self.c1.is_zero()
    }

    pub fn random<R: rand::RngCore>(rng: &mut R) -> Self {
        Self {
            c0: Bls12_381BaseField::random(rng),
            c1: Bls12_381BaseField::random(rng),
        }
    }

    pub fn mul_by_nonresidue(&self) -> Fp2 {
        // Multiply a + bu by u + 1, getting
        // au + a + bu^2 + bu
        // and because u^2 = -1, we get
        // (a - b) + (a + b)u

        Fp2 {
            c0: BLS12_381_BASE.sub(self.c0.clone(), &self.c1),
            c1: BLS12_381_BASE.add(self.c0.clone(), &self.c1),
        }
    }

    pub fn conjugate(&self) -> Self {
        Self {
            c0: self.c0.clone(),
            c1: Bls12_381BaseField::neg(self.c1.clone()),
        }
    }

    /// Raises this element to p.
    /// ref: https://alicebob.cryptoland.net/the-frobenius-endomorphism-with-finite-fields/
    pub fn frobenius_map(&self) -> Self {
        self.conjugate()
    }

    // Operations
    pub fn add(&self, rhs: &Fp2) -> Self {
        Self {
            c0: BLS12_381_BASE.add(self.c0.clone(), &rhs.c0),
            c1: BLS12_381_BASE.add(self.c1.clone(), &rhs.c1),
        }
    }

    pub fn double(&self) -> Self {
        Self {
            c0: BLS12_381_BASE.add(self.c0.clone(), &self.c0),
            c1: BLS12_381_BASE.add(self.c1.clone(), &self.c1),
        }
    }

    pub fn add_base(&self, rhs: &Integer) -> Self {
        Self {
            c0: BLS12_381_BASE.add(self.c0.clone(), rhs),
            c1: BLS12_381_BASE.add(self.c1.clone(), rhs),
        }
    }

    pub fn sub(&self, rhs: &Fp2) -> Self {
        Self {
            c0: BLS12_381_BASE.sub(self.c0.clone(), &rhs.c0),
            c1: BLS12_381_BASE.sub(self.c1.clone(), &rhs.c1),
        }
    }

    pub fn neg(&self) -> Self {
        Self {
            c0: BLS12_381_BASE.neg(self.c0.clone()),
            c1: BLS12_381_BASE.neg(self.c1.clone()),
        }
    }

    pub fn mul(&self, rhs: &Fp2) -> Self {
        // F_{p^2} x F_{p^2} multiplication:
        //   c_0 = a_0 b_0 - a_1 b_1
        //   c_1 = a_0 b_1 + a_1 b_0

        let a0b0 = BLS12_381_BASE.mul(self.c0.clone(), &rhs.c0);
        let a1b1 = BLS12_381_BASE.mul(self.c1.clone(), &rhs.c1);

        let a0b1 = BLS12_381_BASE.mul(self.c0.clone(), &rhs.c1);
        let a1b0 = BLS12_381_BASE.mul(self.c1.clone(), &rhs.c0);

        Self {
            c0: BLS12_381_BASE.sub(a0b0, &a1b1),
            c1: BLS12_381_BASE.add(a0b1, &a1b0),
        }
    }

    pub fn mul_base(&self, base: &Integer) -> Self {
        let a0b = BLS12_381_BASE.mul(self.c0.clone(), base);
        let a1b = BLS12_381_BASE.mul(self.c1.clone(), base);

        Self {
            c0: BLS12_381_BASE.sub(a0b.clone(), &a1b),
            c1: BLS12_381_BASE.add(a0b, &a1b),
        }
    }

    pub fn normalize(&self) -> Self {
        // make sure both c0 and c1 are in the base field
        Self {
            c0: BLS12_381_BASE.reduce(&self.c0),
            c1: BLS12_381_BASE.reduce(&self.c1),
        }
    }

    pub fn mul_by_base(&self, fp: &Integer) -> Self {
        Self {
            c0: BLS12_381_BASE.mul(self.c0.clone(), fp),
            c1: BLS12_381_BASE.mul(self.c1.clone(), fp),
        }
    }

    pub fn from_base(fp: &Integer) -> Self {
        Self {
            c0: fp.clone(),
            c1: Integer::from(0),
        }
    }

    pub fn square(&self) -> Self {
        // Complex squaring for Fp2:
        // c0' = (c0 + c1) * (c0 - c1)
        // c1' = 2 * c0 * c1

        let a = BLS12_381_BASE.add(self.c0.clone(), &self.c1);
        let b = BLS12_381_BASE.sub(self.c0.clone(), &self.c1);
        let c = BLS12_381_BASE.add(self.c0.clone(), &self.c0); // 2 * c0

        Self {
            c0: BLS12_381_BASE.mul(a, &b),
            c1: BLS12_381_BASE.mul(c, &self.c1),
        }
    }

    pub fn cubic(&self) -> Self {
        let square = self.square();
        self.mul(&square)
    }

    /// Computes the multiplicative inverse of this field element.
    /// Returns None if the element is zero.
    pub fn invert(&self) -> Option<Self> {
        // For a + bu, we compute the inverse as (a - bu) / (a^2 + b^2)
        // This uses the identity (a + bu)(a - bu) = a^2 + b^2
        // Note: This requires only one inversion in the base field

        let a_squared = BLS12_381_BASE.square(self.c0.clone());
        let b_squared = BLS12_381_BASE.square(self.c1.clone());
        let sum_of_squares = BLS12_381_BASE.add(a_squared, &b_squared);

        BLS12_381_BASE.invert(sum_of_squares).map(|t| Self {
            c0: BLS12_381_BASE.mul(self.c0.clone(), &t),
            c1: BLS12_381_BASE.neg(BLS12_381_BASE.mul(self.c1.clone(), &t)),
        })
    }

    /// Exponentiation by a large power (variable time)
    pub fn pow(&self, exponent: &Integer) -> Self {
        if exponent.is_zero() {
            return Self::one();
        } else if exponent == &Integer::from(1) {
            return self.clone();
        } else {
            let mut result = self.clone();
            let mut exp = exponent.clone();

            while exp.is_even() {
                result = result.square();
                exp >>= 1;
            }

            if exp.is_one() {
                return result;
            }

            let mut base = result.clone();
            exp >>= 1;

            while !exp.is_zero() {
                base = base.square();
                if exp.is_odd() {
                    result = result.mul(&base);
                }
                exp >>= 1;
            }

            result
        }
    }

    /// Attempts to compute the square root of this element in Fp2.
    /// Returns None if the element is not a quadratic residue.
    /// for p = 3 mod 4
    ///
    /// ref: https://eprint.iacr.org/2012/685.pdf Page 17 Algorithm 9
    ///
    ///
    pub fn sqrt(&self) -> Option<Self> {
        if self.is_zero() {
            return Some(Self::zero());
        }

        // 1: a1 ← a^((q-3)/4)
        let a1 = self.pow(
            &(BLS12_381_BASE
                .fp2_sqrt_constant1
                .clone()
                .expect("sqrt constant1")),
        );

        // 2: α ← a1(a1a)
        let alpha = a1.mul(&a1.mul(self));

        // 3: a0 ← a^q * α
        let a0 = self.frobenius_map().mul(&alpha);

        // 4-6: if a0 = -1 then return false
        if a0.eq(&Self::one().neg()) {
            return None;
        }

        // 7: x0 ← a1a
        let x0 = a1.mul(self);

        // 8-9: if α = -1 then x ← ix0
        if alpha.eq(&Self::one().neg()) {
            return Some(Self {
                c0: BLS12_381_BASE.neg(x0.c1),
                c1: x0.c0,
            });
        } else {
            // 10-12: else b ← (1 + α)^((q-1)/2), x ← bx0
            let b = Self::one().add(&alpha).pow(
                &BLS12_381_BASE
                    .fp2_sqrt_constant2
                    .clone()
                    .expect("sqrt constant1"),
            );
            let x = b.mul(&x0);

            // Verify that x^2 = a
            if x.square().eq(self) {
                Some(x)
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rug::Integer;

    use super::Fp2;

    #[test]
    fn test_sqrt_constants() {
        use num_traits::Pow;

        // (q - 3) / 4
        let p = super::BLS12_381_BASE.modulus();
        let p2: Integer = p.clone() ^ 2 % p.clone();

        let q: Integer = (p2.clone() - 3) / 4;
        println!("q: {:#?}", q);

        let p = super::BLS12_381_BASE.modulus();
        let q: Integer = p.pow(2) - 3;
        let q = q / 4;

        println!("q: {:#?}", q);
    }

    #[test]
    fn test_q_mod_4() {
        let p = super::BLS12_381_BASE.modulus();
        let reminder = p.clone() % 4;

        println!("t: {}", reminder);
    }

    fn gen_a() -> Fp2 {
        let fp2 = super::Fp2::from_u64_vec(
            &[
                0xc9a2_1831_63ee_70d4,
                0xbc37_70a7_196b_5c91,
                0xa247_f8c1_304c_5f44,
                0xb01f_c2a3_726c_80b5,
                0xe1d2_93e5_bbd9_19c9,
                0x04b7_8e80_020e_f2ca,
            ],
            &[
                0x952e_a446_0462_618f,
                0x238d_5edd_f025_c62f,
                0xf6c9_4b01_2ea9_2e72,
                0x03ce_24ea_c1c9_3808,
                0x0559_50f9_45da_483c,
                0x010a_768d_0df4_eabc,
            ],
        );

        fp2.from_mont()
    }

    fn gen_b() -> Fp2 {
        let fp2 = super::Fp2::from_u64_vec(
            &[
                0xa1e0_9175_a4d2_c1fe,
                0x8b33_acfc_204e_ff12,
                0xe244_15a1_1b45_6e42,
                0x61d9_96b1_b6ee_1936,
                0x1164_dbe8_667c_853c,
                0x0788_557a_cc7d_9c79,
            ],
            &[
                0xda6a_87cc_6f48_fa36,
                0x0fc7_b488_277c_1903,
                0x9445_ac4a_dc44_8187,
                0x0261_6d5b_c909_9209,
                0xdbed_4677_2db5_8d48,
                0x11b9_4d50_76c7_b7b1,
            ],
        );

        fp2.from_mont()
    }

    fn gen_c() -> Fp2 {
        let fp2 = super::Fp2::from_u64_vec(
            &[
                0xf597_483e_27b4_e0f7,
                0x610f_badf_811d_ae5f,
                0x8432_af91_7714_327a,
                0x6a9a_9603_cf88_f09e,
                0xf05a_7bf8_bad0_eb01,
                0x0954_9131_c003_ffae,
            ],
            &[
                0x963b_02d0_f93d_37cd,
                0xc95c_e1cd_b30a_73d4,
                0x3087_25fa_3126_f9b8,
                0x56da_3c16_7fab_0d50,
                0x6b50_86b5_f4b6_d6af,
                0x09c3_9f06_2f18_e9f2,
            ],
        );

        fp2.from_mont()
    }

    #[test]
    fn test_square() {
        let a = gen_a();
        let b = gen_b();

        assert_eq!(a.square(), b);
    }

    #[test]
    fn test_mul() {
        let a = gen_a();
        let b = gen_b();

        assert_eq!(a.mul(&a), b);

        let c = gen_c();
        assert_eq!(a.mul(&b), c);
    }

    #[test]
    fn test_add() {
        let a = gen_a();
        let b = gen_b();

        let c = Fp2::from_u64_vec(
            &[
                0x6b82_a9a7_08c1_32d2,
                0x476b_1da3_39ba_5ba4,
                0x848c_0e62_4b91_cd87,
                0x11f9_5955_295a_99ec,
                0xf337_6fce_2255_9f06,
                0x0c3f_e3fa_ce8c_8f43,
            ],
            &[
                0x6f99_2c12_73ab_5bc5,
                0x3355_1366_17a1_df33,
                0x8b0e_f74c_0aed_aff9,
                0x062f_9246_8ad2_ca12,
                0xe146_9770_738f_d584,
                0x12c3_c3dd_84bc_a26d,
            ],
        )
        .from_mont();

        assert_eq!(a.add(&b), c);
    }

    #[test]
    fn test_sub() {
        let a = gen_a();
        let b = gen_b();

        let c = Fp2::from_u64_vec(
            &[
                0xe1c0_86bb_bf1b_5981,
                0x4faf_c3a9_aa70_5d7e,
                0x2734_b5c1_0bb7_e726,
                0xb2bd_7776_af03_7a3e,
                0x1b89_5fb3_98a8_4164,
                0x1730_4aef_6f11_3cec,
            ],
            &[
                0x74c3_1c79_9519_1204,
                0x3271_aa54_79fd_ad2b,
                0xc9b4_7157_4915_a30f,
                0x65e4_0313_ec44_b8be,
                0x7487_b238_5b70_67cb,
                0x0952_3b26_d0ad_19a4,
            ],
        )
        .from_mont();

        assert_eq!(a.sub(&b), c);
    }

    #[test]
    fn test_neg() {
        let a = gen_a();

        let b = Fp2::from_u64_vec(
            &[
                0xf05c_e7ce_9c11_39d7,
                0x6274_8f57_97e8_a36d,
                0xc4e8_d9df_c664_96df,
                0xb457_88e1_8118_9209,
                0x6949_13d0_8772_930d,
                0x1549_836a_3770_f3cf,
            ],
            &[
                0x24d0_5bb9_fb9d_491c,
                0xfb1e_a120_c12e_39d0,
                0x7067_879f_c807_c7b1,
                0x60a9_269a_31bb_dab6,
                0x45c2_56bc_fd71_649b,
                0x18f6_9b5d_2b8a_fbde,
            ],
        )
        .from_mont();

        assert_eq!(a.neg(), b);
    }

    #[test]
    fn test_invert() {
        let a = Fp2::from_u64_vec(
            &[
                0x1128_ecad_6754_9455,
                0x9e7a_1cff_3a4e_a1a8,
                0xeb20_8d51_e08b_cf27,
                0xe98a_d408_11f5_fc2b,
                0x736c_3a59_232d_511d,
                0x10ac_d42d_29cf_cbb6,
            ],
            &[
                0xd328_e37c_c2f5_8d41,
                0x948d_f085_8a60_5869,
                0x6032_f9d5_6f93_a573,
                0x2be4_83ef_3fff_dc87,
                0x30ef_61f8_8f48_3c2a,
                0x1333_f55a_3572_5be0,
            ],
        )
        .from_mont();
        let b = Fp2::from_u64_vec(
            &[
                0x0581_a133_3d4f_48a6,
                0x5824_2f6e_f074_8500,
                0x0292_c955_349e_6da5,
                0xba37_721d_dd95_fcd0,
                0x70d1_6790_3aa5_dfc5,
                0x1189_5e11_8b58_a9d5,
            ],
            &[
                0x0eda_09d2_d7a8_5d17,
                0x8808_e137_a7d1_a2cf,
                0x43ae_2625_c1ff_21db,
                0xf85a_c9fd_f7a7_4c64,
                0x8fcc_dda5_b8da_9738,
                0x08e8_4f0c_b32c_d17d,
            ],
        )
        .from_mont();

        assert!(a.invert().is_some());
        assert_eq!(a.invert().unwrap(), b);

        assert!(b.invert().is_some());
        assert_eq!(b.invert().unwrap(), a);

        let c = Fp2::zero();
        assert!(c.invert().is_none());
    }

    #[test]
    fn test_pow() {
        let a = gen_a();
        let b = gen_b();
        let c = gen_c();

        let a2 = a.pow(&Integer::from(2));
        let a3 = a.pow(&Integer::from(3));

        assert_eq!(a2, b);
        assert_eq!(a3, c);
    }

    #[test]
    fn test_sqrt() {
        let a = Fp2::from_u64_vec(
            &[
                0x2bee_d146_27d7_f9e9,
                0xb661_4e06_660e_5dce,
                0x06c4_cc7c_2f91_d42c,
                0x996d_7847_4b7a_63cc,
                0xebae_bc4c_820d_574e,
                0x1886_5e12_d93f_d845,
            ],
            &[
                0x7d82_8664_baf4_f566,
                0xd17e_6639_96ec_7339,
                0x679e_ad55_cb40_78d0,
                0xfe3b_2260_e001_ec28,
                0x3059_93d0_43d9_1b68,
                0x0626_f03c_0489_b72d,
            ],
        )
        .from_mont();

        assert_eq!(a.sqrt().unwrap().square(), a);

        // b = 5, which is a generator of the p - 1 order
        // multiplicative subgroup
        let b = Fp2::from_u64_vec(
            &[
                0x6631_0000_0010_5545,
                0x2114_0040_0eec_000d,
                0x3fa7_af30_c820_e316,
                0xc52a_8b8d_6387_695d,
                0x9fb4_e61d_1e83_eac5,
                0x005c_b922_afe8_4dc7,
            ],
            &[0, 0, 0, 0, 0, 0],
        );
        assert_eq!(b.sqrt().unwrap().square(), b);

        // c = 25, which is a generator of the (p - 1) / 2 order
        // multiplicative subgroup
        let c = Fp2::from_u64_vec(
            &[
                0x44f6_0000_0051_ffae,
                0x86b8_0141_9948_0043,
                0xd715_9952_f1f3_794a,
                0x755d_6e3d_fe1f_fc12,
                0xd36c_d6db_5547_e905,
                0x02f8_c8ec_bf18_67bb,
            ],
            &[0, 0, 0, 0, 0, 0],
        );
        assert_eq!(c.sqrt().unwrap().square(), c);

        // 2155129644831861015726826462986972654175647013268275306775721078997042729172900466542651176384766902407257452753362*u + 2796889544896299244102912275102369318775038861758288697415827248356648685135290329705805931514906495247464901062529
        // is nonsquare.
        let c = Fp2::from_u64_vec(
            &[
                0xc5fa_1bc8_fd00_d7f6,
                0x3830_ca45_4606_003b,
                0x2b28_7f11_04b1_02da,
                0xa7fb_30f2_8230_f23e,
                0x339c_db9e_e953_dbf0,
                0x0d78_ec51_d989_fc57,
            ],
            &[
                0x27ec_4898_cf87_f613,
                0x9de1_394e_1abb_05a5,
                0x0947_f85d_c170_fc14,
                0x586f_bc69_6b61_14b7,
                0x2b34_75a4_077d_7169,
                0x13e1_c895_cc4b_6c22,
            ],
        );
        assert!(c.sqrt().is_none());
    }
}
