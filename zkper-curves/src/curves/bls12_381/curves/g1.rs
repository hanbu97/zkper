use crate::curves::bls12_381::MILLER_LOOP_CONSTANT;

use self::g1_affine::G1Affine;

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

            // let mul_result = Bls12_381BaseField::mul(x.0.clone(), &Bls12_381BaseField::one());
            // println!("mul_result: {}", mul_result.to_string_radix(16));

            // Convert to projective coordinates and clear cofactor
            // let proj_point = point.to_curve().final_exponentiation();

            // // Ensure the generated point is not the point at infinity
            // if !proj_point.is_identity() {
            //     return proj_point;
            // }
        }
    }
}
