use zkper_curves::curves::bls12_381::curves::{g1_affine::G1Affine, g2_affine::G2Affine};

use super::verification_key::VerificationKey;

/// Preprocessed verification key for proof verification.
#[derive(Clone, Debug)]
pub struct PreparedVerifyingKey {
    /// The original verification key.
    pub vk: VerificationKey,
    /// Pairing result of alpha_g1 and beta_g2.

    /// Negation of gamma_g2, prepared for pairing.
    pub neg_gamma_g2: G2Affine,
    /// Negation of delta_g2, prepared for pairing.
    pub neg_delta_g2: G2Affine,
}
