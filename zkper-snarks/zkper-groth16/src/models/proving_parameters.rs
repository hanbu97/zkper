use std::sync::Arc;

use zkper_curves::curves::bls12_381::curves::{g1_affine::G1Affine, g2_affine::G2Affine};

use super::verification_key::VerificationKey;

/// Parameters generated from the QAP for proving and verifying in the Groth16 zk-SNARK protocol.
#[derive(Clone, Debug)]
pub struct ProvingParameters {
    /// The verification key, containing elements needed for proof verification.
    pub vk: VerificationKey,

    /// H query: ((τ^i * t(τ)) / δ) for i from 0 to m-2.
    /// Used in proving to ensure consistency of τ powers.
    pub h_query: Arc<Vec<G1Affine>>,

    /// L query: (β * u_i(τ) + α * v_i(τ) + w_i(τ)) / δ for all auxiliary inputs.
    /// Used in proving to handle auxiliary (private) inputs.
    pub l_query: Arc<Vec<G1Affine>>,

    /// A query: QAP "A" polynomials evaluated at τ in the Lagrange basis.
    /// Used in proving for the "A" part of the QAP.
    pub a_query: Arc<Vec<G1Affine>>,

    /// B G1 query: QAP "B" polynomials evaluated at τ in G1.
    /// Used in proving for the "B" part of the QAP in G1.
    pub b_g1_query: Arc<Vec<G1Affine>>,

    /// B G2 query: QAP "B" polynomials evaluated at τ in G2.
    /// Used in proving for the "B" part of the QAP in G2.
    pub b_g2_query: Arc<Vec<G2Affine>>,
}
