use zkper_curves::curves::bls12_381::curves::{g1_affine::G1Affine, g2_affine::G2Affine};

/// A verification key for the Groth16 zk-SNARK protocol.
#[derive(Clone, Debug)]
pub struct VerificationKey {
    /// α in G1, used for verifying and creating A/C elements of the proof.
    pub alpha_g1: G1Affine,

    /// β in G1, used for verifying and creating B/C elements of the proof.
    pub beta_g1: G1Affine,

    /// β in G2, used for verifying and creating B/C elements of the proof.
    pub beta_g2: G2Affine,

    /// γ in G2, used for verifying.
    pub gamma_g2: G2Affine,

    /// δ in G1, part of the "trapdoor" for proving and verifying.
    pub delta_g1: G1Affine,

    /// δ in G2, part of the "trapdoor" for proving and verifying.
    pub delta_g2: G2Affine,

    /// IC elements: (β * u_i(τ) + α * v_i(τ) + w_i(τ)) / γ for all public inputs.
    /// These are in G1 and correspond to public input polynomials.
    pub ic: Vec<G1Affine>,
}
