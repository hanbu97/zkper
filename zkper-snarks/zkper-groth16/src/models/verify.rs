use zkper_curves::curves::bls12_381::{
    curves::{g1_affine::G1Affine, g2_affine::G2Affine},
    fields::target::TargetField,
};

/// Preprocessed verification key for proof verification.
#[derive(Clone, Debug)]
pub struct PreparedVerifyingKey {
    /// Pairing result of alpha_g1 and beta_g2.
    pub alpha_g1_beta_g2: TargetField,
    /// Negation of gamma_g2, prepared for pairing.
    pub neg_gamma_g2: G2Affine,
    /// Negation of delta_g2, prepared for pairing.
    pub neg_delta_g2: G2Affine,
    /// IC elements: (β * u_i(τ) + α * v_i(τ) + w_i(τ)) / γ for all public inputs.
    /// These are in G1 and correspond to public input polynomials.
    pub ic: Vec<G1Affine>,
}
