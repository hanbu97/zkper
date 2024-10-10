use std::fmt;

use rand::RngCore;
use rug::Integer;

use crate::{
    backends::montgomery::{INTEGER_FOUR, INTEGER_TWELVE},
    curves::bls12_381::{fields::fp2::Fp2, MILLER_LOOP_CONSTANT, MILLER_LOOP_CONSTANT_IS_NEG},
};

use super::g2_affine::G2Affine;

lazy_static::lazy_static! {
    /// The generators of G1 and G2 are computed by finding the lexicographically smallest valid x-coordinate,
    /// and its lexicographically smallest y-coordinate and scaling it by the cofactor such that the result is not the point at infinity.
    ///
    /// x = 3059144344244213709971259814753781636986470325476647558659373206291635324768958432433509563104347017837885763365758*u
    ///     + 352701069587466618187139116011060144890029952792775240219908644239793785735715026873347600343865175952761926303160
    /// y = 927553665492332455747201965776037880757740193453592970025027978793976877002675564980949289727957565575433344219582*u +
    ///     1985150602287291935568054521177171638300868978215655730859378665066344726373823718423869104263333984641494340347905
    pub static ref G2_GENERATOR_X: Fp2 = {
        let c0 = Integer::from_str_radix(
            "352701069587466618187139116011060144890029952792775240219908644239793785735715026873347600343865175952761926303160",
            10
        ).expect("failed to parse generator integer");
        let c1 = Integer::from_str_radix(
            "3059144344244213709971259814753781636986470325476647558659373206291635324768958432433509563104347017837885763365758",
            10
        ).expect("failed to parse generator integer");
        let x = Fp2::from_integers(c0, c1);
        x
    };
    pub static ref G2_GENERATOR_Y: Fp2 = {
        let c0 = Integer::from_str_radix(
            "1985150602287291935568054521177171638300868978215655730859378665066344726373823718423869104263333984641494340347905",
            10
        ).expect("failed to parse generator integer");
        let c1 = Integer::from_str_radix(
            "927553665492332455747201965776037880757740193453592970025027978793976877002675564980949289727957565575433344219582",
            10
        ).expect("failed to parse generator integer");
        let y = Fp2::from_integers(c0, c1);
        y
    };


    // PSI_X = 1/(u+1)^((p-1)/3)
    pub static ref PSI_X: Fp2 = {
        let c0 = Integer::ZERO;
        let c1 = Integer::from_str_radix(
            "4002409555221667392624310435006688643935503118305586438271171395842971157480381377015405980053539358417135540939437",
            // "3216219264335650953089490096956247703352901229858663303777280467369712744216098189027420766571058176884680122779075", // montgomery form
            10
        ).expect("failed to parse PSI_X integer");
        Fp2::from_integers(c0, c1)
    };

    // PSI_Y = 1/(u+1)^((p-1)/2)
    pub static ref PSI_Y: Fp2 = {
        let c0 = Integer::from_str_radix(
            "2973677408986561043442465346520108879172042883009249989176415018091420807192182638567116318576472649347015917690530",
            // "1821461487266245992767491788684378228087062278322214693001359809350238716280406307949636812899085786271837335624401", // montgomery form
            10
        ).expect("failed to parse PSI_Y integer");
        let c1 = Integer::from_str_radix(
            "1028732146235106349975324479215795277384839936929757896155643118032610843298655225875571310552543014690878354869257",
            // "2180948067955421400650298037051525928469820541616793192330698326773792934210431556493050816229929877766056936935386", // montgomery form
            10
        ).expect("failed to parse PSI_Y integer");
        Fp2::from_integers(c0, c1)
    };

    // PSI_2_X = (u+1)^((1-p^2)/3)
    pub static ref PSI_2_X: Fp2 = {
        let c0 = Integer::from_str_radix(
            "4002409555221667392624310435006688643935503118305586438271171395842971157480381377015405980053539358417135540939436",
            10
        ).expect("failed to parse PSI_Y integer");
        let c1 = Integer::ZERO;
        Fp2::from_integers(c0, c1)
    };
}

/// This is an element of G2 represented in the projective coordinate space.
#[derive(Clone, Debug)]
pub struct G2Projective {
    pub x: Fp2,
    pub y: Fp2,
    pub z: Fp2,
}

impl From<G2Affine> for G2Projective {
    fn from(value: G2Affine) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: Fp2::one(),
        }
    }
}

impl PartialEq for G2Projective {
    fn eq(&self, other: &Self) -> bool {
        // If both points are at infinity, they're equal
        if self.is_identity() && other.is_identity() {
            return true;
        }

        // If only one point is at infinity, they're not equal
        if self.is_identity() || other.is_identity() {
            return false;
        }

        // Compare x and y coordinates in the affine form
        // (x1/z1 == x2/z2) && (y1/z1 == y2/z2)
        // Cross-multiply to avoid division:
        // (x1*z2 == x2*z1) && (y1*z2 == y2*z1)
        (self.x.mul(&other.z) == other.x.mul(&self.z))
            && (self.y.mul(&other.z) == other.y.mul(&self.z))
    }
}

impl Eq for G2Projective {}

impl fmt::Display for G2Projective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(
                f,
                "G2Projective {{\n    x: {},\n    y: {},\n    z: {}\n}}",
                self.x.to_string(),
                self.y.to_string(),
                self.z.to_string(),
            )
        } else {
            write!(
                f,
                "G2Projective({}, {}, {})",
                self.x.to_string(),
                self.y.to_string(),
                self.z.to_string(),
            )
        }
    }
}

impl G2Projective {
    pub fn to_affine(&self) -> G2Affine {
        self.normalize()
    }

    pub fn to_mont(&self) -> G2Projective {
        G2Projective {
            x: self.x.to_mont(),
            y: self.y.to_mont(),
            z: self.z.to_mont(),
        }
    }

    pub fn from_mont(&self) -> G2Projective {
        G2Projective {
            x: self.x.from_mont(),
            y: self.y.from_mont(),
            z: self.z.from_mont(),
        }
    }

    pub fn identity() -> Self {
        Self {
            x: Fp2::zero(),
            y: Fp2::one(),
            z: Fp2::zero(),
        }
    }

    pub fn is_identity(&self) -> bool {
        self.z.is_zero()
    }

    pub fn normalize(&self) -> G2Affine {
        if self.is_identity() {
            return G2Affine::identity();
        }

        // Compute z_inv = 1/z
        let z_inv = self.z.invert().expect("Z should be invertible");

        // Compute x_norm = x * z_inv, y_norm = y * z_inv
        let x_norm = self.x.mul(&z_inv);
        let y_norm = self.y.mul(&z_inv);

        G2Affine {
            x: x_norm,
            y: y_norm,
            infinity: false,
        }
    }

    /// Returns a fixed generator of the group.
    /// The generators of G1 and G2 are computed by finding the lexicographically smallest valid x-coordinate,
    /// and its lexicographically smallest y-coordinate and scaling it by the cofactor such that the result is not the point at infinity.
    ///
    /// x = 3059144344244213709971259814753781636986470325476647558659373206291635324768958432433509563104347017837885763365758*u
    ///     + 352701069587466618187139116011060144890029952792775240219908644239793785735715026873347600343865175952761926303160
    /// y = 927553665492332455747201965776037880757740193453592970025027978793976877002675564980949289727957565575433344219582*u +
    ///     1985150602287291935568054521177171638300868978215655730859378665066344726373823718423869104263333984641494340347905
    pub fn generator() -> Self {
        Self {
            x: G2_GENERATOR_X.clone(),
            y: G2_GENERATOR_Y.clone(),
            z: Fp2::one(),
        }
    }

    /// Computes the doubling of this point.
    /// ref: Algorithm 9, https://eprint.iacr.org/2015/1060.pdf
    ///
    /// Todo: test algorithm used in g1 projective
    pub fn double(&self) -> Self {
        if self.is_identity() {
            return Self::identity();
        }

        let t0 = self.y.square();
        let z3 = t0.clone().add(&t0);
        let z3 = z3.double();
        let z3 = z3.double();
        let t1 = self.y.mul(&self.z);
        let t2 = self.z.square();
        let t2 = t2.mul_base(&INTEGER_TWELVE);
        let x3 = t2.mul(&z3);
        let y3 = t0.add(&t2);
        let z3 = t1.mul(&z3);
        let t1 = t2.double();
        let t2 = t1.add(&t2);
        let t0 = t0.sub(&t2);
        let y3 = t0.mul(&y3);
        let y3 = x3.add(&y3);
        let t1 = self.x.mul(&self.y);
        let x3 = t0.mul(&t1);
        let x3 = x3.double();

        Self {
            x: x3,
            y: y3,
            z: z3,
        }
    }

    /// Adds this point to another point.
    ///
    /// ref: Algorithm 7, https://eprint.iacr.org/2015/1060.pdf
    pub fn add(&self, rhs: &Self) -> Self {
        let t0 = self.x.mul(&rhs.x);
        let t1 = self.y.mul(&rhs.y);
        let t2 = self.z.mul(&rhs.z);
        let t3 = self.x.add(&self.y);
        let t4 = rhs.x.add(&rhs.y);
        let t3 = t3.mul(&t4);
        let t4 = t0.add(&t1);
        let t3 = t3.sub(&t4);
        let t4 = self.y.add(&self.z);
        let x3 = rhs.y.add(&rhs.z);
        let t4 = t4.mul(&x3);
        let x3 = t1.add(&t2);
        let t4 = t4.sub(&x3);
        let x3 = self.x.add(&self.z);
        let y3 = rhs.x.add(&rhs.z);
        let x3 = x3.mul(&y3);
        let y3 = t0.add(&t2);
        let y3 = x3.sub(&y3);
        let x3 = t0.double();
        let t0 = x3.add(&t0);
        let t2 = t2.mul_base(&INTEGER_TWELVE);
        let z3 = t1.add(&t2);
        let t1 = t1.sub(&t2);
        let y3 = y3.mul_base(&INTEGER_TWELVE);
        let x3 = t4.mul(&y3);
        let t2 = t3.mul(&t1);
        let x3 = t2.sub(&x3);
        let y3 = y3.mul(&t0);
        let t1 = t1.mul(&z3);
        let y3 = t1.add(&y3);
        let t0 = t0.mul(&t3);
        let z3 = z3.mul(&t4);
        let z3 = z3.add(&t0);

        G2Projective {
            x: x3,
            y: y3,
            z: z3,
        }
    }

    pub fn add_affine(&self, rhs: &G2Affine) -> Self {
        if rhs.is_identity() {
            return self.clone();
        }

        let t0 = self.x.mul(&rhs.x);
        let t1 = self.y.mul(&rhs.y);
        let t3 = rhs.x.add(&rhs.y);
        let t4 = self.x.add(&self.y);
        let t3 = t3.mul(&t4);
        let t4 = t0.add(&t1);
        let t3 = t3.sub(&t4);
        let t4 = rhs.y.mul(&self.z);
        let t4 = t4.add(&self.y);
        let y3 = rhs.x.mul(&self.z);
        let y3 = y3.add(&self.x);
        let x3 = t0.double();
        let t0 = x3.add(&t0);
        let t2 = self.z.mul_base(&INTEGER_TWELVE);
        let z3 = t1.add(&t2);
        let t1 = t1.sub(&t2);
        let y3 = y3.mul_base(&INTEGER_TWELVE);
        let x3 = t4.mul(&y3);
        let t2 = t3.mul(&t1);
        let x3 = t2.sub(&x3);
        let y3 = y3.mul(&t0);
        let t1 = t1.mul(&z3);
        let y3 = t1.add(&y3);
        let t0 = t0.mul(&t3);
        let z3 = z3.mul(&t4);
        let z3 = z3.add(&t0);

        G2Projective {
            x: x3,
            y: y3,
            z: z3,
        }
    }

    pub fn neg(&self) -> Self {
        G2Projective {
            x: self.x.clone(),
            y: self.y.neg(),
            z: self.z.clone(),
        }
    }

    pub fn sub(&self, rhs: &Self) -> Self {
        self.add(&rhs.neg())
    }

    /// Scalar multiplication of a G1Projective point
    pub fn mul_scalar(&self, scalar: &Integer) -> Self {
        if scalar.is_zero() {
            return G2Projective::identity();
        }

        let mut result = G2Projective::identity();
        let mut temp = self.clone();
        let mut scalar_bits = scalar.clone();

        while !scalar_bits.is_zero() {
            if scalar_bits.is_odd() {
                result = result.add(&temp);
            }
            temp = temp.double();
            scalar_bits >>= 1;
        }

        result
    }

    /// Multiply `self` by `MILLER_LOOP_CONSTANT`, using double and add.
    pub fn mul_by_x(&self) -> G2Projective {
        let mut result = G2Projective::identity();
        // Skip the first bit as it's always 1 for BLS12-381
        let mut x = MILLER_LOOP_CONSTANT >> 1;
        let mut acc = self.clone();

        while x != 0 {
            acc = acc.double();
            if x % 2 == 1 {
                result = result.add(&acc);
            }
            x >>= 1;
        }

        // Apply the sign of x
        if MILLER_LOOP_CONSTANT_IS_NEG {
            result.neg()
        } else {
            result
        }
    }

    /// psi(P) is the untwist-Frobenius-twist endomorhism on E'(Fq2)
    pub fn psi(&self) -> Self {
        let x = self.x.frobenius_map().mul(&PSI_X);
        let y = self.y.frobenius_map().mul(&PSI_Y);

        G2Projective {
            x,
            y,
            z: self.z.frobenius_map(),
        }
    }

    /// For a p-power endomorphism psi(P), compute psi(psi(P))
    pub fn psi2(&self) -> Self {
        G2Projective {
            x: self.x.mul(&PSI_2_X),
            y: self.y.neg(),
            z: self.z.clone(),
        }
    }

    /// Clears the cofactor
    ///
    /// ref: Section 4.1, https://eprint.iacr.org/2017/419.pdf
    /// [h(ψ)]P = [x^2 − x − 1]P + [x − 1]ψ(P) + (ψ^2)(2P)
    pub fn clear_cofactor(&self) -> Self {
        // [x]P
        let x_p = self.mul_by_x();
        // ψ(P)
        let psi_p = self.psi();
        // (ψ^2)(2P)
        let psi2_p2 = self.double().psi2();

        // [x^2]P + [x]ψ(P)
        let t0 = (x_p.add(&psi_p)).mul_by_x();

        (psi2_p2.add(&t0)).sub(&x_p).sub(&psi_p).sub(&self)
    }

    /// Returns a random element in G2
    pub fn random<R: RngCore>(rng: &mut R) -> Self {
        loop {
            let x = Fp2::random(rng);

            let flip_sign = rng.next_u32() % 2 != 0;

            let y_squared = x.cubic().add_base(INTEGER_FOUR);

            let y = y_squared.sqrt();

            if let Some(y) = y {
                let y = if flip_sign { y.neg() } else { y };

                let point = G2Projective {
                    x,
                    y,
                    z: Fp2::one(),
                };

                // clear cofactor
                let proj_point = point.clear_cofactor();

                if !proj_point.is_identity() {
                    return proj_point;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Sub;

    use rug::Integer;
    use zkper_base::rand::ZkperRng;

    use crate::{
        backends::montgomery::{INTEGER_THREE, INTEGER_TWO},
        curves::bls12_381::{
            curves::{g2::G2Projective, g2_affine::G2Affine},
            fields::fp2::Fp2,
            BLS12_381_BASE,
        },
    };

    #[test]
    fn test_g2_random() {
        let mut rng = ZkperRng::new_test();
        let g2 = G2Projective::random(&mut rng);
        println!("g2: {:#}", g2);
    }

    #[test]
    fn test_clear_cofacotr() {
        let point = gen_point();

        let clear_point = point.clear_cofactor();
        println!("clear_point: {:#}", clear_point);

        let g = G2Projective::generator();
        println!("gclear: {:#}", g.clear_cofactor());
    }

    #[test]
    fn test_constants() {
        // generator
        let x = Fp2::from_u64_vec(
            &[
                0xf5f2_8fa2_0294_0a10,
                0xb3f5_fb26_87b4_961a,
                0xa1a8_93b5_3e2a_e580,
                0x9894_999d_1a3c_aee9,
                0x6f67_b763_1863_366b,
                0x0581_9192_4350_bcd7,
            ],
            &[
                0xa5a9_c075_9e23_f606,
                0xaaa0_c59d_bccd_60c3,
                0x3bb1_7e18_e286_7806,
                0x1b1a_b6cc_8541_b367,
                0xc2b6_ed0e_f215_8547,
                0x1192_2a09_7360_edf3,
            ],
        )
        .from_mont();
        let y = Fp2::from_u64_vec(
            &[
                0x4c73_0af8_6049_4c4a,
                0x597c_fa1f_5e36_9c5a,
                0xe7e6_856c_aa0a_635a,
                0xbbef_b5e9_6e0d_495f,
                0x07d3_a975_f0ef_25a2,
                0x0083_fd8e_7e80_dae5,
            ],
            &[
                0xadc0_fc92_df64_b05d,
                0x18aa_270a_2b14_61dc,
                0x86ad_ac6a_3be4_eba0,
                0x7949_5c4e_c93d_a33a,
                0xe717_5850_a43c_caed,
                0x0b2b_c2a1_63de_1bf2,
            ],
        )
        .from_mont();

        println!("x: {:#?}", x);
        println!("y: {:#?}", y);

        // psi
        let x = Fp2::from_u64_vec(
            &[0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
            &[
                0x890dc9e4867545c3,
                0x2af322533285a5d5,
                0x50880866309b7e2c,
                0xa20d1b8c7e881024,
                0x14e4f04fe2db9068,
                0x14e56d3f1564853a,
            ],
        );
        let y = Fp2::from_u64_vec(
            &[
                0x3e2f585da55c9ad1,
                0x4294213d86c18183,
                0x382844c88b623732,
                0x92ad2afd19103e18,
                0x1d794e4fac7cf0b9,
                0x0bd592fc7d825ec8,
            ],
            &[
                0x7bcfa7a25aa30fda,
                0xdc17dec12a927e7c,
                0x2f088dd86b4ebef1,
                0xd1ca2087da74d4a7,
                0x2da2596696cebc1d,
                0x0e2b7eedbbfd87d2,
            ],
        );

        println!("x: {:#?}", x);
        println!("y: {:#?}", y);
    }

    fn gen_z() -> Fp2 {
        Fp2::from_u64_vec(
            &[
                0x0ef2ddffab187c0a,
                0x2424522b7d5ecbfc,
                0xc6f341a3398054f4,
                0x5523ddf409502df0,
                0xd55c0b5a88e0dd97,
                0x066428d704923e52,
            ],
            &[
                0x538bbe0c95b4878d,
                0xad04a50379522881,
                0x6d5c05bf5c12fb64,
                0x4ce4a069a2d34787,
                0x59ea6c8d0dffaeaf,
                0x0d42a083a75bd6f3,
            ],
        )
        .from_mont()
    }

    fn gen_point() -> G2Projective {
        let z = gen_z();

        let point = G2Projective {
            x: Fp2::from_u64_vec(
                &[
                    0xee4c8cb7c047eaf2,
                    0x44ca22eee036b604,
                    0x33b3affb2aefe101,
                    0x15d3e45bbafaeb02,
                    0x7bfc2154cd7419a4,
                    0x0a2d0c2b756e5edc,
                ],
                &[
                    0xfc224361029a8777,
                    0x4cbf2baab8740924,
                    0xc5008c6ec6592c89,
                    0xecc2c57b472a9c2d,
                    0x8613eafd9d81ffb1,
                    0x10fe54daa2d3d495,
                ],
            )
            .from_mont()
            .mul(&z),
            y: Fp2::from_u64_vec(
                &[
                    0x7de7edc43953b75c,
                    0x58be1d2de35e87dc,
                    0x5731d30b0e337b40,
                    0xbe93b60cfeaae4c9,
                    0x8b22c203764bedca,
                    0x01616c8d1033b771,
                ],
                &[
                    0xea126fe476b5733b,
                    0x85cee68b5dae1652,
                    0x98247779f7272b04,
                    0xa649c8b468c6e808,
                    0xb5b9a62dff0c4e45,
                    0x1555b67fc7bbe73d,
                ],
            )
            .from_mont(),
            z: z.square().mul(&z),
        };
        point
    }

    #[test]
    fn test_psi() {
        let generator = G2Projective::generator();
        assert_eq!(generator.psi().psi(), generator.psi2());

        // let z = gen_z();
        let point = gen_point();
        let point_psi = point.psi();

        let point_psi_ref = G2Projective {
            x: Fp2::from_hexs("0x028d12205f3f90aeaa99b20ab214d9959e8511a4ca3d31ed9286a3aa0f26244008c42b2833d849c848850a58a95e3de0", "0x078e97a2d79e0aca781f3bebcac2368d8b0c83efa65e01a2d83635311f03a6b011ad5a3e456c68707de0be169ff868c1"),
            y: Fp2::from_hexs("0x092dfbc26538d387e1fc9d8be2dc941a270151dce92bf8d15357d16f4f5ce55f78c9d2613207b1c22a9032e94b41dd01", "0x18bc79aecea296ea540ead5842b18fb7fac6bc1ea005ade90b2e51a1d06b24abddc3ad5b910cea7466d2aa4eb47a53ab"),
            z: Fp2::from_hexs("0x09f4f08a58010cb973926f7a0fd533ce65a96ffbd83f16b8322a6a8015ebf3ddc7a9d1fd61c08fc7eb1681f703b031d5", "0x16b484190c1c7d7790b719c86fa37d460033d985342581453d566d13864bbb4681f2d729b606d0d990547be0c99fc491"),
        };

        assert_eq!(point_psi, point_psi_ref);
        assert_eq!(point.psi().psi(), point.psi2());

        assert_eq!(point.psi().double(), point.double().psi());
        assert_eq!(
            point.psi().add(&generator.psi()),
            (point.add(&generator)).psi()
        );
    }

    #[test]
    fn test_add() {
        let a = G2Projective::identity();
        let b = G2Projective::identity();
        let c = a.add(&b);

        assert!(c.is_identity());

        // println!("c: {:#}", c);
        let b = G2Projective::generator();
        let z = Fp2::from_u64_vec(
            &[
                0xba7a_fa1f_9a6f_e250,
                0xfa0f_5b59_5eaf_e731,
                0x3bdc_4776_94c3_06e7,
                0x2149_be4b_3949_fa24,
                0x64aa_6e06_49b2_078c,
                0x12b1_08ac_3364_3c3e,
            ],
            &[
                0x1253_25df_3d35_b5a8,
                0xdc46_9ef5_555d_7fe3,
                0x02d7_16d2_4431_06a9,
                0x05a1_db59_a6ff_37d0,
                0x7cf7_784e_5300_bb8f,
                0x16a8_8922_c7a5_e844,
            ],
        )
        .from_mont();

        let b = G2Projective {
            x: b.x.mul(&z),
            y: b.y.mul(&z),
            z,
        };

        let c = a.add(&b);
        println!("c: {:#}", c);

        assert_eq!(c, b);
    }

    #[test]
    fn test_double() {
        let id = G2Projective::identity();
        let t = id.double();
        println!("t: {:#}", t);

        let g = G2Projective::generator();
        let t = g.double();
        println!("t: {:#}", t);

        let g2_affine_ref = G2Affine {
            x: Fp2::from_u64_vec(
                &[
                    0xe9d9_e2da_9620_f98b,
                    0x54f1_1993_46b9_7f36,
                    0x3db3_b820_376b_ed27,
                    0xcfdb_31c9_b0b6_4f4c,
                    0x41d7_c127_8635_4493,
                    0x0571_0794_c255_c064,
                ],
                &[
                    0xd6c1_d3ca_6ea0_d06e,
                    0xda0c_bd90_5595_489f,
                    0x4f53_52d4_3479_221d,
                    0x8ade_5d73_6f8c_97e0,
                    0x48cc_8433_925e_f70e,
                    0x08d7_ea71_ea91_ef81,
                ],
            ),
            y: Fp2::from_u64_vec(
                &[
                    0x15ba_26eb_4b0d_186f,
                    0x0d08_6d64_b7e9_e01e,
                    0xc8b8_48dd_652f_4c78,
                    0xeecf_46a6_123b_ae4f,
                    0x255e_8dd8_b6dc_812a,
                    0x1641_42af_21dc_f93f,
                ],
                &[
                    0xf9b4_a1a8_9598_4db4,
                    0xd417_b114_cccf_f748,
                    0x6856_301f_c89f_086e,
                    0x41c7_7787_8931_e3da,
                    0x3556_b155_066a_2105,
                    0x00ac_f7d3_25cb_89cf,
                ],
            ),
            infinity: false,
        };

        let g2_affine = G2Affine::from(&t).to_montgomery();
        assert_eq!(g2_affine, g2_affine_ref);
    }

    #[test]
    fn test_g2_generator() {
        let t = G2Projective::generator();
        println!("t: {:#}", t);
    }

    #[test]
    fn test_psi_constants() {
        // currently not work

        use num_traits::Pow;
        let modulus = BLS12_381_BASE.modulus();
        let x = Integer::from(MILLER_LOOP_CONSTANT);
        pub const MILLER_LOOP_CONSTANT: u64 = 0xd201_0000_0001_0000;
        let p_squared = modulus.clone().pow(2);

        // Calculate (p-1)/3 and (p-1)/2
        let p_minus_1_div_3 = (p_squared.clone().sub(Integer::ONE)).div_exact(&INTEGER_THREE);
        let p_minus_1_div_2 = (p_squared.clone().sub(Integer::ONE)).div_exact(&INTEGER_TWO);

        println!("p_minus_1_div_3: {:#?}", p_minus_1_div_3);
        println!("p_minus_1_div_2: {:#?}", p_minus_1_div_2);

        // Calculate x + 1
        let x_plus_1: Integer = x.clone() + 1;

        // PSI_CONSTANT_0 = 1/(x+1)^((p-1)/3) mod p
        let psi_constant_0 = x_plus_1
            .clone()
            .pow_mod(&p_minus_1_div_3, &modulus)
            .unwrap()
            .invert(&modulus)
            .unwrap();

        // PSI_CONSTANT_1 = 1/(x+1)^((p-1)/2) mod p
        let psi_constant_1 = x_plus_1
            .clone()
            .pow_mod(&p_minus_1_div_2, &modulus)
            .unwrap()
            .invert(&modulus)
            .unwrap();

        // DOUBLE_PSI_CONSTANT_0 = (x+1)^((1-p^2)/3) mod p
        let one_minus_p_squared_div_3 = (Integer::from(1) - &p_squared) / 3;
        let double_psi_constant_0 = x_plus_1
            .pow_mod(&one_minus_p_squared_div_3, &modulus)
            .unwrap();

        // Convert to Fp2
        let psi_constant_0_fp2 = Fp2::from_integers(Integer::from(0), psi_constant_0);
        let psi_constant_1_fp2 = Fp2::from_integers(psi_constant_1, Integer::from(0));
        let double_psi_constant_0_fp2 = Fp2::from_integers(double_psi_constant_0, Integer::from(0));

        println!("psi_constant_0: {:#?}", psi_constant_0_fp2);
        println!("psi_constant_1: {:#?}", psi_constant_1_fp2);
        println!("double_psi_constant_0: {:#?}", double_psi_constant_0_fp2);
    }
}
