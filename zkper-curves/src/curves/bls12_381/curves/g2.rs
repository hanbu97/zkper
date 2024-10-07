use std::fmt;

use rand::RngCore;
use rug::Integer;
use zkper_base::rand::ZkperRng;

use crate::{
    backends::montgomery::INTEGER_FOUR,
    curves::bls12_381::{fields::fp2::Fp2, MILLER_LOOP_CONSTANT},
};

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
    pub fn identity() -> Self {
        Self {
            x: Fp2::zero(),
            y: Fp2::one(),
            z: Fp2::zero(),
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
