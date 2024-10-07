use crate::{curves::bls12_381::BLS12_381_BASE, traits::field::FieldTrait};
use rug::Integer;
use std::fmt::Display;

use super::base::Bls12_381BaseField;

#[derive(Clone, Debug, PartialEq)]
pub struct Fp2 {
    pub c0: Integer, // Base field element c0
    pub c1: Integer, // Base field element c1
}

impl Display for Fp2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {})",
            self.c0.to_string_radix(16),
            self.c1.to_string_radix(16)
        )
    }
}

impl Fp2 {
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

    pub fn from_monterey(&self) -> Self {
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

    pub fn is_zero(&self) -> bool {
        self.c0.is_zero() && self.c1.is_zero()
    }

    pub fn random<R: rand::RngCore>(rng: &mut R) -> Self {
        Self {
            c0: Bls12_381BaseField::random(rng),
            c1: Bls12_381BaseField::random(rng),
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
}

#[cfg(test)]
mod tests {
    use crate::curves::bls12_381::Bls12_381BaseField;

    use super::Fp2;

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

        fp2.from_monterey()
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

        fp2.from_monterey()
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

        fp2.from_monterey()
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
        .from_monterey();

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
        .from_monterey();

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
        .from_monterey();

        assert_eq!(a.neg(), b);
    }
}
