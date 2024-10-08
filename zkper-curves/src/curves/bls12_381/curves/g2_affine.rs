use std::fmt::{self, Display};

use crate::curves::bls12_381::fields::fp2::Fp2;

use super::g2::{G2Projective, G2_GENERATOR_X, G2_GENERATOR_Y};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct G2Affine {
    pub x: Fp2,
    pub y: Fp2,
    pub infinity: bool,
}

impl<'a> From<&'a G2Projective> for G2Affine {
    fn from(p: &'a G2Projective) -> G2Affine {
        let zinv = p.z.invert().unwrap_or(Fp2::zero());

        if zinv.is_zero() {
            return G2Affine::identity();
        }

        let x = p.x.mul(&zinv);
        let y = p.y.mul(&zinv);

        G2Affine {
            x,
            y,
            infinity: false,
        }
    }
}

impl From<G2Projective> for G2Affine {
    fn from(p: G2Projective) -> G2Affine {
        G2Affine::from(&p)
    }
}

impl Display for G2Affine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(
                f,
                "G2Affine {{\n    x: {},\n    y: {},\n    infinity: {}\n}}",
                self.x, self.y, self.infinity
            )
        } else {
            write!(f, "G2Affine({}, {}, {})", self.x, self.y, self.infinity)
        }
    }
}

impl G2Affine {
    pub fn from_montgomery(p: &G2Affine) -> G2Affine {
        G2Affine {
            x: Fp2::from_mont(&p.x),
            y: Fp2::from_mont(&p.y),
            infinity: p.infinity,
        }
    }

    pub fn to_montgomery(&self) -> G2Affine {
        G2Affine {
            x: Fp2::to_mont(&self.x),
            y: Fp2::to_mont(&self.y),
            infinity: self.infinity,
        }
    }

    /// Returns the identity of the group: the point at infinity.
    pub fn identity() -> Self {
        G2Affine {
            x: Fp2::zero(),
            y: Fp2::one(),
            infinity: true,
        }
    }

    pub fn is_identity(&self) -> bool {
        self.infinity
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
            infinity: false,
        }
    }
}
