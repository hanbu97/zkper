use crate::{
    backends::montgomery::{INTEGER_THREE, INTEGER_TWO},
    curves::bls12_381::{BLS12_381_BASE, MILLER_LOOP_CONSTANT},
};

use self::g1_affine::G1Affine;
use rug::{ops::Pow, Complete};

use super::*;

#[derive(Clone, Debug)]
pub struct G1Projective {
    pub x: Integer,
    pub y: Integer,
    pub z: Integer,
}

impl<'a> From<&'a G1Affine> for G1Projective {
    fn from(p: &'a G1Affine) -> G1Projective {
        G1Projective {
            x: p.x.clone(),
            y: p.y.clone(),
            z: if p.infinity {
                Bls12_381BaseField::zero()
            } else {
                Bls12_381BaseField::one()
            },
        }
    }
}

impl From<(rug::Integer, rug::Integer, rug::Integer)> for G1Projective {
    fn from(p: (rug::Integer, rug::Integer, rug::Integer)) -> G1Projective {
        G1Projective {
            x: p.0,
            y: p.1,
            z: p.2,
        }
    }
}

impl fmt::Display for G1Projective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(
                f,
                "G1Projective {{\n    x: {},\n    y: {},\n    z: {}\n}}",
                self.x.to_string_radix(16),
                self.y.to_string_radix(16),
                self.z.to_string_radix(16),
            )
        } else {
            write!(
                f,
                "G1Projective({}, {}, {})",
                self.x.to_string_radix(16),
                self.y.to_string_radix(16),
                self.z.to_string_radix(16),
            )
        }
    }
}

impl G1Projective {
    pub fn new(x: Integer, y: Integer, z: Integer) -> Self {
        Self { x, y, z }
    }

    pub fn to_tuple(&self) -> (Integer, Integer, Integer) {
        (self.x.clone(), self.y.clone(), self.z.clone())
    }
}

impl G1Projective {
    /// Returns the identity element (point at infinity).
    pub fn identity() -> Self {
        G1Projective {
            x: Bls12_381BaseField::zero(),
            y: Bls12_381BaseField::one(),
            z: Bls12_381BaseField::zero(),
        }
    }

    /// Returns true if this element is the identity (the point at infinity).
    #[inline]
    pub fn is_identity(&self) -> bool {
        self.z.is_zero()
    }

    pub fn neg(&self) -> G1Projective {
        G1Projective {
            x: self.x.clone(),
            y: Bls12_381BaseField::neg(self.y.clone()),
            z: self.z.clone(),
        }
    }

    pub fn double(&self) -> Self {
        BLS12_381_BASE
            .double_standard(&self.x, &self.y, &self.z)
            .into()
    }

    // /// Frobenius endomorphism of the point
    // /// This operation is crucial in the context of the Miller loop for BLS12-381
    // pub fn frobenius_map(&self) -> G1Projective {
    //     let mut result = G1Projective::identity();
    //     let mut temp = self.clone();

    //     // Skip the first bit as it's always 1 for BLS12-381
    //     let mut x = MILLER_LOOP_CONSTANT >> 1;

    //     while x != 0 {
    //         temp = temp.double();

    //         if x % 2 == 1 {
    //             result = result.add(&temp);
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

    pub fn random<R: RngCore>(rng: &mut R) -> Self {
        // loop {
        let x: Bls12_381BaseField = rng.gen();

        let flip_sign = rng.next_u32() % 2 != 0;

        // Compute y = sqrt(x^3 + 4)
        let y_squared = Bls12_381BaseField::cubic(x.0.clone()) + 4;
        let y_squared = Bls12_381BaseField::sqrt(y_squared);

        if let Some(y) = y_squared {
            let y = if flip_sign {
                Bls12_381BaseField::neg(y)
            } else {
                y
            };

            // Create affine point
            let point = G1Projective {
                x: x.0,
                y,
                z: Bls12_381BaseField::one(),
            };

            println!("point: {}", point);

            // Convert to projective coordinates and clear cofactor
            // let proj_point = point.to_curve().final_exponentiation();

            // // Ensure the generated point is not the point at infinity
            // if !proj_point.is_identity() {
            //     return proj_point;
            // }
        }

        Self::identity()

        // }
    }
}

#[cfg(test)]
mod tests {
    use zkper_base::rand::ZkperRng;

    use crate::{
        backends::montgomery::{INTEGER_EIGHT, INTEGER_FOUR},
        curves::bls12_381::{self, BLS12_381_BASE, BLS12_381_SCALAR},
    };

    use super::*;

    #[test]
    fn test_g1_projective_random() {
        let mut rng = ZkperRng::new_test();

        let x: Bls12_381BaseField = rng.gen();
        let flip_sign = rng.next_u32() % 2 != 0;

        println!("x: {}", x);
        println!("flip_sign: {}", flip_sign);

        // Compute y = sqrt(x^3 + 4)
        let y_squared = Bls12_381BaseField::cubic(x.0.clone()) + 4;
        let y_squared = Bls12_381BaseField::sqrt(y_squared);

        if let Some(y) = y_squared {
            println!("y_squared: {}", y.to_string_radix(16));

            let y = if flip_sign {
                Bls12_381BaseField::neg(y)
            } else {
                y
            };

            // Create affine point
            let point = G1Projective {
                x: x.0,
                y,
                z: Bls12_381BaseField::one(),
            };

            println!("point: {:#}", point);

            println!();

            let point_double = point.double();
            println!("point_double: {:#}", point_double);

            println!();

            let x_mont = BLS12_381_BASE.to_montgomery(&point.x);
            let y_mont = BLS12_381_BASE.to_montgomery(&point.y);
            let z_mont = BLS12_381_BASE.to_montgomery(&point.z);

            let point_add =
                BLS12_381_BASE.add_mont(&x_mont, &y_mont, &z_mont, &x_mont, &y_mont, &z_mont);

            let x_raw = BLS12_381_BASE.from_montgomery(&point_add.0);
            let y_raw = BLS12_381_BASE.from_montgomery(&point_add.1);
            let z_raw = BLS12_381_BASE.from_montgomery(&point_add.2);

            println!(
                "point_add: {:#?}",
                (
                    x_raw.to_string_radix(16),
                    y_raw.to_string_radix(16),
                    z_raw.to_string_radix(16)
                )
            );

            // println!("point_prime: {:#}", point_prime);

            println!();
        }
    }
}
