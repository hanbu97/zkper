use std::fmt::Display;

use zkper_curves::curves::bls12_381::curves::{g1_affine::G1Affine, g2_affine::G2Affine};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Proof {
    pub a: G1Affine,
    pub b: G2Affine,
    pub c: G1Affine,
}

impl Display for Proof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "a: {}\n, b: {}\n, c: {}", self.a, self.b, self.c)
    }
}

// impl Proof {
//     pub fn to_bytes(&self) -> Vec<u8> {
//         let mut bytes = vec![];
//         bytes.extend_from_slice(&self.a.to_bytes());
//         bytes.extend_from_slice(&self.b.to_bytes());
//         bytes.extend_from_slice(&self.c.to_bytes());
//         bytes
//     }
// }
