use std::fmt::{self, Display};

use super::{fp2::Fp2, fp6::Fp6};

lazy_static::lazy_static! {
    // Fp2::NONRESIDUE^(((q^1) - 1) / 6)
    pub static ref FROBENIUS_COEFF_FP12_C1: Fp2 =Fp2::from_strs(
        "3850754370037169011952147076051364057158807420970682438676050522613628423219637725072182697113062777891589506424760",
        "151655185184498381465642749684540099398075398968325446656007613510403227271200139370504932015952886146304766135027"
    );
}

/// Represents an element of Fp12 as c0 + c1 * w
/// where w is the cubic non-residue in Fp6.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fp12 {
    pub c0: Fp6,
    pub c1: Fp6,
}

impl Display for Fp12 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fp12(\n{} \n+ ({}) * w\n)", self.c0, self.c1)
    }
}

impl Fp12 {
    /// Creates a new Fp12 element from two Fp6 elements.
    pub fn new(c0: Fp6, c1: Fp6) -> Self {
        Fp12 { c0, c1 }
    }

    /// Returns the zero element of Fp12.
    pub fn zero() -> Self {
        Fp12 {
            c0: Fp6::zero(),
            c1: Fp6::zero(),
        }
    }

    /// Checks if the element is zero.
    pub fn is_zero(&self) -> bool {
        self.c0.is_zero() && self.c1.is_zero()
    }

    /// Returns the multiplicative identity element of Fp12.
    pub fn one() -> Self {
        Fp12 {
            c0: Fp6::one(),
            c1: Fp6::zero(),
        }
    }

    pub fn is_one(&self) -> bool {
        self.c0.is_one() && self.c1.is_zero()
    }

    /// Generates a random element of Fp12.
    pub fn random<R: rand::RngCore>(rng: &mut R) -> Self {
        Fp12 {
            c0: Fp6::random(rng),
            c1: Fp6::random(rng),
        }
    }

    /// Computes the conjugate of this element.
    pub fn conjugate(&self) -> Self {
        Fp12 {
            c0: self.c0.clone(),
            c1: self.c1.neg(),
        }
    }

    /// Multiplies two Fp12 elements.
    pub fn mul(&self, other: &Fp12) -> Fp12 {
        let aa = self.c0.mul(&other.c0);
        let bb = self.c1.mul(&other.c1);
        let o = other.c0.add(&other.c1);
        let c1 = self.c1.add(&self.c0);
        let c1 = c1.mul(&o);
        let c1 = c1.sub(&aa);
        let c1 = c1.sub(&bb);
        let c0 = bb.mul_by_nonresidue();
        let c0 = c0.add(&aa);

        Fp12 { c0, c1 }
    }

    /// opt mul only c0, c1, c4
    pub fn mul_by_c0_c1_c4(&self, c0: &Fp2, c1: &Fp2, c4: &Fp2) -> Fp12 {
        let aa = self.c0.mul_by_c0_c1(c0, c1);
        let bb = self.c1.mul_by_c1(c4);
        let o = c1.add(c4);
        let c1 = self.c1.add(&self.c0);
        let c1 = c1.mul_by_c0_c1(c0, &o);
        let c1 = c1.sub(&aa).sub(&bb);
        let c0 = bb.mul_by_nonresidue().add(&aa);

        Fp12 { c0, c1 }
    }

    /// Adds two Fp12 elements.
    pub fn add(&self, other: &Fp12) -> Fp12 {
        Fp12 {
            c0: self.c0.add(&other.c0),
            c1: self.c1.add(&other.c1),
        }
    }

    /// Subtracts one Fp12 element from another.
    pub fn sub(&self, other: &Fp12) -> Fp12 {
        Fp12 {
            c0: self.c0.sub(&other.c0),
            c1: self.c1.sub(&other.c1),
        }
    }

    /// Negates this Fp12 element.
    pub fn neg(&self) -> Fp12 {
        Fp12 {
            c0: self.c0.neg(),
            c1: self.c1.neg(),
        }
    }

    /// Computes the multiplicative inverse of this element, if it exists.
    pub fn invert(&self) -> Option<Self> {
        if self.is_zero() {
            None
        } else {
            let t = (self.c0.square().sub(&self.c1.square().mul_by_nonresidue())).invert()?;
            Some(Fp12 {
                c0: self.c0.mul(&t),
                c1: self.c1.mul(&t).neg(),
            })
        }
    }

    /// Computes the square of this element.
    pub fn square(&self) -> Self {
        let ab = self.c0.mul(&self.c1);
        let c0c1 = self.c0.add(&self.c1);
        let c0 = self.c1.mul_by_nonresidue();
        let c0 = c0.add(&self.c0);
        let c0 = c0.mul(&c0c1);
        let c0 = c0.sub(&ab);
        let c1 = ab.add(&ab);
        let c0 = c0.sub(&ab.mul_by_nonresidue());

        Fp12 { c0, c1 }
    }

    /// Raises this element to p.
    pub fn frobenius_map(&self) -> Self {
        let c0 = self.c0.frobenius_map();
        let c1 = self.c1.frobenius_map();

        // c1 = c1 * (u + 1)^((p - 1) / 6)
        let c1 = c1.mul(&Fp6::from(FROBENIUS_COEFF_FP12_C1.clone()));

        Fp12 { c0, c1 }
    }
}

#[cfg(test)]
mod tests {
    use rug::Integer;

    use crate::curves::bls12_381::fields::{fp2::Fp2, fp6::Fp6};

    use super::Fp12;

    #[test]
    fn test_constants() {
        let t = Integer::from_str_radix(
            "1904d3bf02bb0667c231beb4202c0d1f0fd603fd3cbd5f4f7b2443d784bab9c4f67ea53d63e7813d8d0775ed92235fb8",
             16
            ).unwrap();
        println!("{}", t);

        let t = Integer::from_str_radix(
            "fc3e2b36c4e03288e9e902231f9fb854a14787b6c7b36fec0c8ec971f63c5f282d5ac14d6c7ec22cf78a126ddc4af3",
            16,
        ).unwrap();
        println!("{}", t);
    }

    #[test]
    fn test_operations() {
        let a = Fp12 {
            c0: Fp6 {
                c0: Fp2::from_u64_vec(
                    &[
                        0x47f9_cb98_b1b8_2d58,
                        0x5fe9_11eb_a3aa_1d9d,
                        0x96bf_1b5f_4dd8_1db3,
                        0x8100_d27c_c925_9f5b,
                        0xafa2_0b96_7464_0eab,
                        0x09bb_cea7_d8d9_497d,
                    ],
                    &[
                        0x0303_cb98_b166_2daa,
                        0xd931_10aa_0a62_1d5a,
                        0xbfa9_820c_5be4_a468,
                        0x0ba3_643e_cb05_a348,
                        0xdc35_34bb_1f1c_25a6,
                        0x06c3_05bb_19c0_e1c1,
                    ],
                )
                .from_mont(),
                c1: Fp2::from_u64_vec(
                    &[
                        0x46f9_cb98_b162_d858,
                        0x0be9_109c_f7aa_1d57,
                        0xc791_bc55_fece_41d2,
                        0xf84c_5770_4e38_5ec2,
                        0xcb49_c1d9_c010_e60f,
                        0x0acd_b8e1_58bf_e3c8,
                    ],
                    &[
                        0x8aef_cb98_b15f_8306,
                        0x3ea1_108f_e4f2_1d54,
                        0xcf79_f69f_a1b7_df3b,
                        0xe4f5_4aa1_d16b_1a3c,
                        0xba5e_4ef8_6105_a679,
                        0x0ed8_6c07_97be_e5cf,
                    ],
                )
                .from_mont(),
                c2: Fp2::from_u64_vec(
                    &[
                        0xcee5_cb98_b15c_2db4,
                        0x7159_1082_d23a_1d51,
                        0xd762_30e9_44a1_7ca4,
                        0xd19e_3dd3_549d_d5b6,
                        0xa972_dc17_01fa_66e3,
                        0x12e3_1f2d_d6bd_e7d6,
                    ],
                    &[
                        0xad2a_cb98_b173_2d9d,
                        0x2cfd_10dd_0696_1d64,
                        0x0739_6b86_c6ef_24e8,
                        0xbd76_e2fd_b1bf_c820,
                        0x6afe_a7f6_de94_d0d5,
                        0x1099_4b0c_5744_c040,
                    ],
                )
                .from_mont(),
            },
            c1: Fp6 {
                c0: Fp2::from_u64_vec(
                    &[
                        0x47f9_cb98_b1b8_2d58,
                        0x5fe9_11eb_a3aa_1d9d,
                        0x96bf_1b5f_4dd8_1db3,
                        0x8100_d27c_c925_9f5b,
                        0xafa2_0b96_7464_0eab,
                        0x09bb_cea7_d8d9_497d,
                    ],
                    &[
                        0x0303_cb98_b166_2daa,
                        0xd931_10aa_0a62_1d5a,
                        0xbfa9_820c_5be4_a468,
                        0x0ba3_643e_cb05_a348,
                        0xdc35_34bb_1f1c_25a6,
                        0x06c3_05bb_19c0_e1c1,
                    ],
                )
                .from_mont(),
                c1: Fp2::from_u64_vec(
                    &[
                        0x46f9_cb98_b162_d858,
                        0x0be9_109c_f7aa_1d57,
                        0xc791_bc55_fece_41d2,
                        0xf84c_5770_4e38_5ec2,
                        0xcb49_c1d9_c010_e60f,
                        0x0acd_b8e1_58bf_e3c8,
                    ],
                    &[
                        0x8aef_cb98_b15f_8306,
                        0x3ea1_108f_e4f2_1d54,
                        0xcf79_f69f_a1b7_df3b,
                        0xe4f5_4aa1_d16b_1a3c,
                        0xba5e_4ef8_6105_a679,
                        0x0ed8_6c07_97be_e5cf,
                    ],
                )
                .from_mont(),
                c2: Fp2::from_u64_vec(
                    &[
                        0xcee5_cb98_b15c_2db4,
                        0x7159_1082_d23a_1d51,
                        0xd762_30e9_44a1_7ca4,
                        0xd19e_3dd3_549d_d5b6,
                        0xa972_dc17_01fa_66e3,
                        0x12e3_1f2d_d6bd_e7d6,
                    ],
                    &[
                        0xad2a_cb98_b173_2d9d,
                        0x2cfd_10dd_0696_1d64,
                        0x0739_6b86_c6ef_24e8,
                        0xbd76_e2fd_b1bf_c820,
                        0x6afe_a7f6_de94_d0d5,
                        0x1099_4b0c_5744_c040,
                    ],
                )
                .from_mont(),
            },
        };

        let b = Fp12 {
            c0: Fp6 {
                c0: Fp2::from_u64_vec(
                    &[
                        0x47f9_cb98_b1b8_2d58,
                        0x5fe9_11eb_a3aa_1d9d,
                        0x96bf_1b5f_4dd8_1db3,
                        0x8100_d272_c925_9f5b,
                        0xafa2_0b96_7464_0eab,
                        0x09bb_cea7_d8d9_497d,
                    ],
                    &[
                        0x0303_cb98_b166_2daa,
                        0xd931_10aa_0a62_1d5a,
                        0xbfa9_820c_5be4_a468,
                        0x0ba3_643e_cb05_a348,
                        0xdc35_34bb_1f1c_25a6,
                        0x06c3_05bb_19c0_e1c1,
                    ],
                )
                .from_mont(),
                c1: Fp2::from_u64_vec(
                    &[
                        0x46f9_cb98_b162_d858,
                        0x0be9_109c_f7aa_1d57,
                        0xc791_bc55_fece_41d2,
                        0xf84c_5770_4e38_5ec2,
                        0xcb49_c1d9_c010_e60f,
                        0x0acd_b8e1_58bf_e348,
                    ],
                    &[
                        0x8aef_cb98_b15f_8306,
                        0x3ea1_108f_e4f2_1d54,
                        0xcf79_f69f_a1b7_df3b,
                        0xe4f5_4aa1_d16b_1a3c,
                        0xba5e_4ef8_6105_a679,
                        0x0ed8_6c07_97be_e5cf,
                    ],
                )
                .from_mont(),
                c2: Fp2::from_u64_vec(
                    &[
                        0xcee5_cb98_b15c_2db4,
                        0x7159_1082_d23a_1d51,
                        0xd762_30e9_44a1_7ca4,
                        0xd19e_3dd3_549d_d5b6,
                        0xa972_dc17_01fa_66e3,
                        0x12e3_1f2d_d6bd_e7d6,
                    ],
                    &[
                        0xad2a_cb98_b173_2d9d,
                        0x2cfd_10dd_0696_1d64,
                        0x0739_6b86_c6ef_24e8,
                        0xbd76_e2fd_b1bf_c820,
                        0x6afe_a7f6_de94_d0d5,
                        0x1099_4b0c_5744_c040,
                    ],
                )
                .from_mont(),
            },
            c1: Fp6 {
                c0: Fp2::from_u64_vec(
                    &[
                        0x47f9_cb98_b1b8_2d58,
                        0x5fe9_11eb_a3aa_1d9d,
                        0x96bf_1b5f_4dd2_1db3,
                        0x8100_d27c_c925_9f5b,
                        0xafa2_0b96_7464_0eab,
                        0x09bb_cea7_d8d9_497d,
                    ],
                    &[
                        0x0303_cb98_b166_2daa,
                        0xd931_10aa_0a62_1d5a,
                        0xbfa9_820c_5be4_a468,
                        0x0ba3_643e_cb05_a348,
                        0xdc35_34bb_1f1c_25a6,
                        0x06c3_05bb_19c0_e1c1,
                    ],
                )
                .from_mont(),
                c1: Fp2::from_u64_vec(
                    &[
                        0x46f9_cb98_b162_d858,
                        0x0be9_109c_f7aa_1d57,
                        0xc791_bc55_fece_41d2,
                        0xf84c_5770_4e38_5ec2,
                        0xcb49_c1d9_c010_e60f,
                        0x0acd_b8e1_58bf_e3c8,
                    ],
                    &[
                        0x8aef_cb98_b15f_8306,
                        0x3ea1_108f_e4f2_1d54,
                        0xcf79_f69f_a117_df3b,
                        0xe4f5_4aa1_d16b_1a3c,
                        0xba5e_4ef8_6105_a679,
                        0x0ed8_6c07_97be_e5cf,
                    ],
                )
                .from_mont(),
                c2: Fp2::from_u64_vec(
                    &[
                        0xcee5_cb98_b15c_2db4,
                        0x7159_1082_d23a_1d51,
                        0xd762_30e9_44a1_7ca4,
                        0xd19e_3dd3_549d_d5b6,
                        0xa972_dc17_01fa_66e3,
                        0x12e3_1f2d_d6bd_e7d6,
                    ],
                    &[
                        0xad2a_cb98_b173_2d9d,
                        0x2cfd_10dd_0696_1d64,
                        0x0739_6b86_c6ef_24e8,
                        0xbd76_e2fd_b1bf_c820,
                        0x6afe_a7f6_de94_d0d5,
                        0x1099_4b0c_5744_c040,
                    ],
                )
                .from_mont(),
            },
        };

        let c = Fp12 {
            c0: Fp6 {
                c0: Fp2::from_u64_vec(
                    &[
                        0x47f9_cb98_71b8_2d58,
                        0x5fe9_11eb_a3aa_1d9d,
                        0x96bf_1b5f_4dd8_1db3,
                        0x8100_d27c_c925_9f5b,
                        0xafa2_0b96_7464_0eab,
                        0x09bb_cea7_d8d9_497d,
                    ],
                    &[
                        0x0303_cb98_b166_2daa,
                        0xd931_10aa_0a62_1d5a,
                        0xbfa9_820c_5be4_a468,
                        0x0ba3_643e_cb05_a348,
                        0xdc35_34bb_1f1c_25a6,
                        0x06c3_05bb_19c0_e1c1,
                    ],
                )
                .from_mont(),
                c1: Fp2::from_u64_vec(
                    &[
                        0x46f9_cb98_b162_d858,
                        0x0be9_109c_f7aa_1d57,
                        0x7791_bc55_fece_41d2,
                        0xf84c_5770_4e38_5ec2,
                        0xcb49_c1d9_c010_e60f,
                        0x0acd_b8e1_58bf_e3c8,
                    ],
                    &[
                        0x8aef_cb98_b15f_8306,
                        0x3ea1_108f_e4f2_1d54,
                        0xcf79_f69f_a1b7_df3b,
                        0xe4f5_4aa1_d16b_133c,
                        0xba5e_4ef8_6105_a679,
                        0x0ed8_6c07_97be_e5cf,
                    ],
                )
                .from_mont(),
                c2: Fp2::from_u64_vec(
                    &[
                        0xcee5_cb98_b15c_2db4,
                        0x7159_1082_d23a_1d51,
                        0xd762_40e9_44a1_7ca4,
                        0xd19e_3dd3_549d_d5b6,
                        0xa972_dc17_01fa_66e3,
                        0x12e3_1f2d_d6bd_e7d6,
                    ],
                    &[
                        0xad2a_cb98_b173_2d9d,
                        0x2cfd_10dd_0696_1d64,
                        0x0739_6b86_c6ef_24e8,
                        0xbd76_e2fd_b1bf_c820,
                        0x6afe_a7f6_de94_d0d5,
                        0x1099_4b0c_1744_c040,
                    ],
                )
                .from_mont(),
            },
            c1: Fp6 {
                c0: Fp2::from_u64_vec(
                    &[
                        0x47f9_cb98_b1b8_2d58,
                        0x5fe9_11eb_a3aa_1d9d,
                        0x96bf_1b5f_4dd8_1db3,
                        0x8100_d27c_c925_9f5b,
                        0xafa2_0b96_7464_0eab,
                        0x09bb_cea7_d8d9_497d,
                    ],
                    &[
                        0x0303_cb98_b166_2daa,
                        0xd931_10aa_0a62_1d5a,
                        0xbfa9_820c_5be4_a468,
                        0x0ba3_643e_cb05_a348,
                        0xdc35_34bb_1f1c_25a6,
                        0x06c3_05bb_19c0_e1c1,
                    ],
                )
                .from_mont(),
                c1: Fp2::from_u64_vec(
                    &[
                        0x46f9_cb98_b162_d858,
                        0x0be9_109c_f7aa_1d57,
                        0xc791_bc55_fece_41d2,
                        0xf84c_5770_4e38_5ec2,
                        0xcb49_c1d3_c010_e60f,
                        0x0acd_b8e1_58bf_e3c8,
                    ],
                    &[
                        0x8aef_cb98_b15f_8306,
                        0x3ea1_108f_e4f2_1d54,
                        0xcf79_f69f_a1b7_df3b,
                        0xe4f5_4aa1_d16b_1a3c,
                        0xba5e_4ef8_6105_a679,
                        0x0ed8_6c07_97be_e5cf,
                    ],
                )
                .from_mont(),
                c2: Fp2::from_u64_vec(
                    &[
                        0xcee5_cb98_b15c_2db4,
                        0x7159_1082_d23a_1d51,
                        0xd762_30e9_44a1_7ca4,
                        0xd19e_3dd3_549d_d5b6,
                        0xa972_dc17_01fa_66e3,
                        0x12e3_1f2d_d6bd_e7d6,
                    ],
                    &[
                        0xad2a_cb98_b173_2d9d,
                        0x2cfd_10dd_0696_1d64,
                        0x0739_6b86_c6ef_24e8,
                        0xbd76_e2fd_b1bf_c820,
                        0x6afe_a7f6_de94_d0d5,
                        0x1099_4b0c_5744_1040,
                    ],
                )
                .from_mont(),
            },
        };

        let a = a.square().invert().unwrap().square().add(&c);
        let b = b.square().invert().unwrap().square().add(&a);
        let c = c.square().invert().unwrap().square().add(&b);

        assert_eq!(a.square(), a.mul(&a));
        assert_eq!(b.square(), b.mul(&b));
        assert_eq!(c.square(), c.mul(&c));

        assert_eq!(
            a.add(&b).mul(&c.square()),
            c.clone().mul(&c).mul(&a).add(&(c.clone().mul(&c).mul(&b)))
        );

        assert_eq!(
            a.invert().unwrap().mul(&b.invert().unwrap()),
            a.clone().mul(&b).invert().unwrap()
        );

        assert!(a != a.frobenius_map());

        assert_eq!(
            a,
            a.frobenius_map()
                .frobenius_map()
                .frobenius_map()
                .frobenius_map()
                .frobenius_map()
                .frobenius_map()
                .frobenius_map()
                .frobenius_map()
                .frobenius_map()
                .frobenius_map()
                .frobenius_map()
                .frobenius_map()
        );
    }
}
