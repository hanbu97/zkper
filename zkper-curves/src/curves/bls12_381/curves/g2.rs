use crate::curves::bls12_381::fields::fp2::Fp2;

/// This is an element of G2 represented in the projective coordinate space.
#[derive(Clone, Debug)]
pub struct G2Projective {
    pub x: Fp2,
    pub y: Fp2,
    pub z: Fp2,
}

impl G2Projective {
    pub fn identity() -> Self {
        Self {
            x: Fp2::zero(),
            y: Fp2::one(),
            z: Fp2::zero(),
        }
    }
}
