use crate::{
    backends::montgomery::{INTEGER_THREE, INTEGER_TWO},
    curves::bls12_381::MILLER_LOOP_CONSTANT,
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

    pub fn double(&self) -> Self {
        if self.is_identity() {
            return self.clone();
        }

        let modulus = Bls12_381BaseField::modulus();

        let xx = Bls12_381BaseField::square(self.x.clone());
        let yy = Bls12_381BaseField::square(self.y.clone());
        let zz = Bls12_381BaseField::square(self.z.clone());
        let xy = Bls12_381BaseField::mul(self.x.clone(), &self.y);

        let t = xx * INTEGER_THREE % modulus;

        let x3 = {
            let xy2 = (xy.clone() * INTEGER_TWO) % modulus;
            ((&t * &t) - xy2.clone() - xy2) % modulus
        };

        let y3 = {
            let yy2 = (&yy * &yy).complete() % modulus;
            let yy8 = (yy2 * 8u32) % modulus;
            ((&t * (xy - &x3)) - yy8) % modulus
        };

        let z3 = {
            let y_plus_z = (&self.y + &self.z).complete() % modulus;
            ((&y_plus_z * &y_plus_z) - yy - &zz) % modulus
        };

        G1Projective {
            x: x3,
            y: y3,
            z: z3,
        }

        // let xy2 = (self.x.clone() * &self.y) * 2;
        // let xyyy = self.x.clone() * &yy * 3;

        // let a: Integer = yy * 3;
        // let b: Integer = xyyy * 2;
        // let c = xx * 3;

        // let y = &a * (b.clone() - c);
        // let x: Integer = &a.pow(2) - b * 2;
        // let z = xy2 * &self.z * 2;

        // G1Projective {
        //     x: x % Bls12_381BaseField::modulus(),
        //     y: y % Bls12_381BaseField::modulus(),
        //     z: z % Bls12_381BaseField::modulus(),
        // }
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

            // let s = INTEGER_FOUR * point.x.clone() * point.y.clone().square()
            //     % Bls12_381BaseField::modulus();
            // let m = (INTEGER_THREE * point.x.clone().square()
            //     + INTEGER_FOUR * point.z.clone().square().square())
            //     % Bls12_381BaseField::modulus();
            // let t = (m.clone().square() - INTEGER_TWO * s.clone()) % Bls12_381BaseField::modulus();

            // let x_prime = t.clone();
            // let y_prime = (m * (s - t) - INTEGER_EIGHT * point.y.clone().square().square())
            //     % Bls12_381BaseField::modulus();
            // let z_prime =
            //     (INTEGER_TWO * point.y.clone() * point.z.clone()) % Bls12_381BaseField::modulus();

            // let (x_prime, y_prime, z_prime) = double_g1_jacobian(&point);

            // println!();

            // println!("x_prime: {:#}", x_prime.to_string_radix(16));
            // println!("y_prime: {:#}", y_prime.to_string_radix(16));
            // println!("z_prime: {:#}", z_prime.to_string_radix(16));

            println!();

            let modulus = Bls12_381BaseField::modulus();

            let a =
                Bls12_381BaseField::mul(Bls12_381BaseField::square(point.x.clone()), INTEGER_THREE)
                    + Bls12_381BaseField::mul(
                        Bls12_381BaseField::square(point.z.clone()),
                        INTEGER_FOUR,
                    );

            let b = Bls12_381BaseField::mul(point.y.clone(), &point.z) * INTEGER_TWO % modulus;

            let y2 = Bls12_381BaseField::square(point.y.clone());
            let c = Bls12_381BaseField::mul(y2.clone(), &point.x) * INTEGER_FOUR;

            let d = Bls12_381BaseField::square(y2) * INTEGER_EIGHT % modulus;

            let c8 = Bls12_381BaseField::mul(c.clone(), &INTEGER_EIGHT);
            let x = Bls12_381BaseField::sub(Bls12_381BaseField::square(a.clone()), &c8);
            let x = Bls12_381BaseField::mul(x, &b);

            let c4 = Bls12_381BaseField::mul(c, &INTEGER_FOUR);
            let a2 = Bls12_381BaseField::square(a.clone());
            let c4_sub_a2 = Bls12_381BaseField::sub(c4, &a2);
            let y = Bls12_381BaseField::mul(a, &c4_sub_a2);

            let d2 = Bls12_381BaseField::mul(d.clone(), &INTEGER_TWO);
            let y = Bls12_381BaseField::sub(y, &d2);

            let z = BLS12_381_BASE.cubic(b);

            println!("x: {:#}", x.to_string_radix(16));
            println!("y: {:#}", y.to_string_radix(16));
            println!("z: {:#}", z.to_string_radix(16));

            println!();
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
