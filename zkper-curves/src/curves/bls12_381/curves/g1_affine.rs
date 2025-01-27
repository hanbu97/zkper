use self::g1::{G1Projective, G1_GENERATOR_X, G1_GENERATOR_Y};

use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct G1Affine {
    pub x: Integer,
    pub y: Integer,
    pub infinity: bool,
}

impl G1Affine {
    pub fn generator() -> Self {
        Self {
            x: G1_GENERATOR_X.clone(),
            y: G1_GENERATOR_Y.clone(),
            infinity: false,
        }
    }

    /// Returns true if this point is the point at infinity.
    #[inline]
    pub fn is_identity(&self) -> bool {
        self.infinity
    }

    /// Returns the identity element (point at infinity).
    #[inline]
    pub fn identity() -> Self {
        G1Affine {
            x: Bls12_381BaseField::zero(),
            y: Bls12_381BaseField::one(),
            infinity: true,
        }
    }

    // /// Scalar multiplication of a G1Projective point
    // pub fn mul_scalar(&self, scalar: &Integer) -> Self {
    //     if scalar.is_zero() {
    //         return Self::identity();
    //     }

    //     let mut result = Self::identity();
    //     let mut temp = self.clone();
    //     let mut scalar_bits = scalar.clone();

    //     while !scalar_bits.is_zero() {
    //         if scalar_bits.is_odd() {
    //             result = result.add(&temp);
    //         }
    //         temp = temp.double();
    //         scalar_bits >>= 1;
    //     }

    //     result
    // }
}

impl<'a> From<&'a G1Projective> for G1Affine {
    fn from(p: &'a G1Projective) -> G1Affine {
        let zinv = Bls12_381BaseField::invert(p.z.clone()).unwrap_or(Integer::ZERO);
        let x = Bls12_381BaseField::mul(p.x.clone(), &zinv);
        let y = Bls12_381BaseField::mul(p.y.clone(), &zinv);

        if zinv.is_zero() {
            G1Affine::identity()
        } else {
            G1Affine {
                x,
                y,
                infinity: false,
            }
        }
    }
}

impl From<G1Projective> for G1Affine {
    fn from(p: G1Projective) -> G1Affine {
        G1Affine::from(&p)
    }
}

impl fmt::Display for G1Affine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "G1Affine {{\n    x: {},\n    y: {},\n    infinity: {}\n}}",
            self.x.to_string_radix(16),
            self.y.to_string_radix(16),
            self.infinity
        )
    }
}

impl G1Affine {
    pub fn new(x: Integer, y: Integer, infinity: bool) -> Self {
        Self { x, y, infinity }
    }

    /// to G1Projective
    pub fn to_curve(&self) -> G1Projective {
        self.into()
    }
}
