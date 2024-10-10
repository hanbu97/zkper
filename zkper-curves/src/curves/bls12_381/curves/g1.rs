use crate::{
    backends::montgomery::INTEGER_FOUR,
    curves::bls12_381::{BLS12_381_BASE, MILLER_LOOP_CONSTANT, MILLER_LOOP_CONSTANT_IS_NEG},
};

use self::g1_affine::G1Affine;

use super::*;

#[derive(Clone, Debug)]
pub struct G1Projective {
    pub x: Integer,
    pub y: Integer,
    pub z: Integer,
}

impl PartialEq for G1Projective {
    fn eq(&self, other: &Self) -> bool {
        let self_normalized = self.normalize();
        let other_normalized = other.normalize();

        self_normalized.x == other_normalized.x
            && self_normalized.y == other_normalized.y
            && self_normalized.z == other_normalized.z
    }
}

impl Eq for G1Projective {}

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

    pub fn to_affine(&self) -> G1Affine {
        self.into()
    }

    pub fn from_str_hex(x: &str, y: &str, z: &str) -> Self {
        Self {
            x: Integer::from_str_radix(x.strip_prefix("0x").unwrap_or(x), 16).unwrap(),
            y: Integer::from_str_radix(y.strip_prefix("0x").unwrap_or(y), 16).unwrap(),
            z: Integer::from_str_radix(z.strip_prefix("0x").unwrap_or(z), 16).unwrap(),
        }
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

    /// Returns the identity element (point at infinity). in montgomery form
    pub fn identity_mont() -> Self {
        G1Projective {
            x: Bls12_381BaseField::zero(),
            y: Bls12_381BaseField::one(),
            z: Bls12_381BaseField::zero(),
        }
        .to_montgomery()
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

    /// to montgomery form
    pub fn to_montgomery(&self) -> G1Projective {
        G1Projective {
            x: BLS12_381_BASE.to_montgomery(&self.x),
            y: BLS12_381_BASE.to_montgomery(&self.y),
            z: BLS12_381_BASE.to_montgomery(&self.z),
        }
    }

    /// from montgomery form
    pub fn from_montgomery(&self) -> G1Projective {
        G1Projective {
            x: BLS12_381_BASE.from_montgomery(&self.x),
            y: BLS12_381_BASE.from_montgomery(&self.y),
            z: BLS12_381_BASE.from_montgomery(&self.z),
        }
    }

    /// normalize
    pub fn normalize(&self) -> G1Projective {
        BLS12_381_BASE.normalize(&self.x, &self.y, &self.z).into()
    }

    /// Double this point
    pub fn double(&self) -> Self {
        BLS12_381_BASE
            .double_standard(&self.x, &self.y, &self.z)
            .into()
    }

    /// Double this point in montgomery form
    pub fn double_mont(&self) -> Self {
        BLS12_381_BASE.double_mont(&self.x, &self.y, &self.z).into()
    }

    /// Add this point to another point
    pub fn add(&self, other: &G1Projective) -> Self {
        BLS12_381_BASE
            .add_standard(&self.x, &self.y, &self.z, &other.x, &other.y, &other.z)
            .into()
    }

    /// Add this point to another point in montgomery form
    pub fn add_mont(&self, other: &G1Projective) -> Self {
        BLS12_381_BASE
            .add_mont(&self.x, &self.y, &self.z, &other.x, &other.y, &other.z)
            .into()
    }

    /// Subtract another point from this point
    pub fn sub(&self, rhs: &G1Projective) -> G1Projective {
        self.add(&rhs.neg())
    }

    /// Subtract another point from this point in montgomery form
    pub fn sub_mont(&self, rhs: &G1Projective) -> G1Projective {
        self.add_mont(&rhs.neg())
    }

    /// Scalar multiplication of a G1Projective point
    pub fn mul_scalar(&self, scalar: &Integer) -> Self {
        if scalar.is_zero() {
            return G1Projective::identity();
        }

        let mut result = G1Projective::identity();
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

    /// Scalar multiplication of a G1Projective point in Montgomery form using binary expansion method
    pub fn mul_scalar_mont(&self, scalar: &Integer) -> Self {
        if scalar.is_zero() {
            return G1Projective::identity_mont();
        }

        let mut result = G1Projective::identity_mont();
        let mut temp = self.clone();
        let mut scalar_bits = scalar.clone();

        while !scalar_bits.is_zero() {
            if scalar_bits.is_odd() {
                result = result.add_mont(&temp);
            }
            temp = temp.double_mont();
            scalar_bits >>= 1;
        }

        result
    }

    /// Frobenius endomorphism of the point
    /// This operation is crucial in the context of the Miller loop for BLS12-381
    pub fn frobenius_map(&self) -> G1Projective {
        let mut result = G1Projective::identity();
        let mut temp = self.clone();

        // Skip the first bit as it's always 1 for BLS12-381
        let mut x = MILLER_LOOP_CONSTANT >> 1;

        while x != 0 {
            temp = temp.double();

            if x % 2 == 1 {
                result = result.add(&temp);
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

    /// Frobenius endomorphism of the point in montgomery form
    /// This operation is crucial in the context of the Miller loop for BLS12-381
    pub fn frobenius_map_mont(&self) -> G1Projective {
        let mut result = G1Projective::identity_mont();
        let mut temp = self.clone();

        // Skip the first bit as it's always 1 for BLS12-381
        let mut x = MILLER_LOOP_CONSTANT >> 1;

        while x != 0 {
            temp = temp.double_mont();

            if x % 2 == 1 {
                result = result.add_mont(&temp);
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

    /// Final exponentiation in the context of the Miller loop
    /// This operation ensures that the point is in the correct subgroup
    ///
    /// Multiplies by $(1 - z)$, where $z$ is the parameter of BLS12-381, which
    /// [suffices to clear](https://ia.cr/2019/403) the cofactor and map
    /// elliptic curve points to elements of $\mathbb{G}\_1$.
    pub fn final_exponentiation(&self) -> G1Projective {
        self.sub(&self.frobenius_map())
    }

    /// Final exponentiation in the context of the Miller loop in montgomery form
    pub fn final_exponentiation_mont(&self) -> G1Projective {
        self.sub_mont(&self.frobenius_map_mont())
    }

    /// Returns a random element in G1
    pub fn random<R: RngCore>(rng: &mut R) -> Self {
        loop {
            let x: Bls12_381BaseField = rng.gen();

            let flip_sign = rng.next_u32() % 2 != 0;

            // Compute y = sqrt(x^3 + 4)
            let y_squared = Bls12_381BaseField::cubic(x.0.clone()) + INTEGER_FOUR;
            let y_squared = Bls12_381BaseField::sqrt(y_squared);

            if let Some(y) = y_squared {
                let y = if flip_sign {
                    Bls12_381BaseField::neg(y)
                } else {
                    y
                };

                let point = G1Projective {
                    x: x.0,
                    y,
                    z: Bls12_381BaseField::one(),
                };

                // clear cofactor
                let proj_point = point.final_exponentiation();

                // Ensure the generated point is not the point at infinity
                if !proj_point.is_identity() {
                    return proj_point;
                }
            }
        }
    }

    /// Returns a random element in G1 in montgomery form
    pub fn random_mont<R: RngCore>(rng: &mut R) -> Self {
        loop {
            let x = Bls12_381BaseField::random_mont(rng);

            let flip_sign = rng.next_u32() % 2 != 0;

            // Compute y = sqrt(x^3 + 4)
            let y_squared = BLS12_381_BASE.add(
                BLS12_381_BASE.mont_cubic(&x),
                &BLS12_381_BASE.to_montgomery(&INTEGER_FOUR),
            );

            let y_squared = BLS12_381_BASE.mont_sqrt(&y_squared);

            if let Some(y) = y_squared {
                let y = if flip_sign {
                    Bls12_381BaseField::neg(y)
                } else {
                    y
                };

                // Create affine point
                let point = G1Projective {
                    x,
                    y,
                    z: Bls12_381BaseField::r().clone(),
                };

                // clear cofactor
                let proj_point = point.final_exponentiation_mont();

                // Ensure the generated point is not the point at infinity
                if !proj_point.is_identity() {
                    return proj_point;
                }
            }
        }
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
    fn test_add() {
        let p1 = G1Projective::from_str_hex(
            "0x04aa8444f5077e437fbc394ff94e0c40647be6a1fe7f1997be46b6612a579b026cfa2a62db4f358994b0a261a97744a4",
            "0x113741a656f80e3496990fed5fd7bc0eec83736b16183eb89aaae7589292741f8e451c5e84677821785c6e874380b878",
            "0x1092dad1a4779aec52001864933b85c230161cfe3d0aeee37c77e6c795c0254750ac035e38cead477205186848232f78",
        );
        let p2 = G1Projective::from_str_hex(
            "0x11459e239a20dd72af811e89f44c6f60e7c8724f873a7485ca2fcad73e761c91a8700dd71be3bd474793c341b2d589fb",
            "0x10530f94537969f0cdee7911af42ade2917cc7a00bfdb879d756882b20bab9ff69d626ace7e8e8cddc3aaeb0f8ddbe33",
            "0x0e878d2fb01a461c980c708ecb1bd38be832f32d4dbf63b8891087823a3ab79c335bdf7436664a0f5946b13c22b63510",
        );

        let p3 = p1.add(&p2);

        let p1_mont = p1.to_montgomery();
        let p2_mont = p2.to_montgomery();
        let p3_mont = p1_mont.add_mont(&p2_mont);

        let p3_raw = p3_mont.from_montgomery();

        println!("p3: {:#}", p3);
        println!("p3_raw: {:#}", p3_raw);

        let p3_norm = p3.normalize();
        let p3_raw_norm = p3_raw.normalize();

        println!("p3_norm: {:#}", p3_norm);
        println!("p3_raw_norm: {:#}", p3_raw_norm);
    }

    #[test]
    fn test_g1_projective_random() {
        let mut rng = ZkperRng::new_test();

        let x: Bls12_381BaseField = rng.gen();
        let flip_sign = rng.next_u32() % 2 != 0;

        println!("x: {}", x);
        println!("flip_sign: {}", flip_sign);

        // Compute y = sqrt(x^3 + 4)
        let y_squared = Bls12_381BaseField::cubic(x.0.clone()) + INTEGER_FOUR;
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

            let point_mont = point.to_montgomery();

            println!();

            let point_identity = G1Projective::identity();
            println!("point_identity: {:#}", point_identity);

            let point_identity_mont = point_identity.to_montgomery();

            let point_add_mont = point_mont.add_mont(&point_identity_mont);
            let point_add_mont_raw = point_add_mont.from_montgomery();

            println!("point_add_mont_raw: {:#}", point_add_mont_raw);

            let point_add_mont_raw_norm = point_add_mont_raw.normalize();
            println!("point_add_mont_raw_norm: {:#}", point_add_mont_raw_norm);

            println!();
            println!();

            let p1 = BLS12_381_BASE.new_element(point_add_mont_raw.x);
            let p2 = BLS12_381_BASE.new_element(point_add_mont_raw.y);
            let p3 = BLS12_381_BASE.new_element(point_add_mont_raw.z);

            println!("p1: {:#}", p1.to_string_radix(16));
            println!("p2: {:#}", p2.to_string_radix(16));
            println!("p3: {:#}", p3.to_string_radix(16));

            println!();
            println!();

            let point_add = point.add(&point_identity);
            println!("point_add: {:#}", point_add);
        }
    }
}

#[test]
fn test_g1projective_mul_scalar() {
    let scalar = Integer::from_str_radix(
        "591000f3eede48cbd3c9b75e12b60acaa3457bcbe22c5157e2cbef2cea9a43ca",
        16,
    )
    .unwrap();

    let g1 = G1Projective::from_str_hex(
        "14fff6cd2d99a3e655f79afaccbb1393a3ae62139ffebd0d580bcb1ef66ef480767e3dc81e7a2a1e726c5e4100973ce5", 
        "b5525cc92832d25a58a05caf2dd5f19a784b30ca27cddcdb497cf4b346fd5f384da2557c59c6fcfc350e58cfe1fd78f", 
        "461e83d5dcef3cced2cba68661f99b6d19df5ccb367578dce250033e9f64310e93be9f44c4e5d311a081d99ecd5402a"
    );

    let g1_scalar = g1.mul_scalar(&scalar);
    println!("{:#}", g1_scalar);

    let out_ref = G1Projective::from_str_hex(
        "6bef549f07f386bfef60eca04d9357fef2e198aab5e4a4cfa70ae64932da740b2941236a5b4a1e416b8b46b848ea192", 
        "16d7b00c7e32c33e81fad9d1cfdc0e67b3590a53c0927775009f77a8d1dc6d0bdbef72ab5de1a5ab095d4f95ec5312ef", 
        "44376697afdeac3df23a6a34753b65752ba49b39a40731d658bcefaf51d9e19b1e8e612484e50599c439f2469d34c06"
    );

    let out_norm = g1_scalar.normalize();
    let out_ref_norm = out_ref.normalize();

    println!("{:#}", out_norm);
    println!("{:#}", out_ref_norm);
}

// {
//   x: 0x06bef549f07f386bfef60eca04d9357fef2e198aab5e4a4cfa70ae64932da740b2941236a5b4a1e416b8b46b848ea192,
//   y: 0x16d7b00c7e32c33e81fad9d1cfdc0e67b3590a53c0927775009f77a8d1dc6d0bdbef72ab5de1a5ab095d4f95ec5312ef,
//   z: 0x044376697afdeac3df23a6a34753b65752ba49b39a40731d658bcefaf51d9e19b1e8e612484e50599c439f2469d34c06
// }
