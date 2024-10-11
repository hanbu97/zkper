use zkper_curves::curves::bls12_381::curves::{g1_affine::G1Affine, g2_affine::G2Affine};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Proof {
    pub a: G1Affine,
    pub b: G2Affine,
    pub c: G1Affine,
}
