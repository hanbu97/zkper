use std::fmt;

use rand::RngCore;
use rug::Integer;
use zkper_base::rand::ZkperRng;

use crate::{
    backends::montgomery::{
        INTEGER_EIGHT, INTEGER_FOUR, INTEGER_THREE, INTEGER_TWELVE, INTEGER_TWO,
    },
    curves::bls12_381::{fields::fp2::Fp2, MILLER_LOOP_CONSTANT},
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
}

/// This is an element of G2 represented in the projective coordinate space.
#[derive(Clone, Debug)]
pub struct G2Projective {
    pub x: Fp2,
    pub y: Fp2,
    pub z: Fp2,
}

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

    // /// Multiply `self` by `MILLER_LOOP_CONSTANT`, using double and add.
    // fn mul_by_x(&self) -> G2Projective {
    //     let mut result = G2Projective::identity();
    //     // Skip the first bit as it's always 1 for BLS12-381
    //     let mut x = MILLER_LOOP_CONSTANT >> 1;
    //     let mut acc = self.clone();

    //     while x != 0 {
    //         acc = acc.double();
    //         if x % 2 == 1 {
    //             result = result.add(&acc);
    //         }
    //         x >>= 1;
    //     }

    //     // Apply the sign of x
    //     if MILLER_LOOP_CONSTANT_IS_NEG {
    //         result.neg()
    //     } else {
    //         result
    //     }
    // }

    /// Returns a random element in G2
    pub fn random<R: RngCore>(rng: &mut R) -> Self {
        let x = Fp2::random(rng);

        let flip_sign = rng.next_u32() % 2 != 0;

        let y_squared = x.cubic().add_base(INTEGER_FOUR);
        println!("y_squared: {:#}", y_squared);

        let y = y_squared.sqrt();

        if let Some(y) = y {
            println!("y: {:#}", y);

            let y = if flip_sign { y.neg() } else { y };

            println!("y: {:#}", y);

            let point = G2Projective {
                x,
                y,
                z: Fp2::one(),
            };

            println!("point: {:#}", point);

            // clear cofactor
            // let proj_point = point.final_exponentiation();
        }

        Self::identity()
    }
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
fn test_g2_random() {
    let mut rng = ZkperRng::new_test();
    let g2 = G2Projective::random(&mut rng);
    // println!("g2: {:#?}", g2);
}
