use zkper_curves::curves::bls12_381::paring::BLS12_381Pairing;

use crate::models::{verification_key::VerificationKey, verify::PreparedVerifyingKey};

pub fn prepare_verifying_key(vk: &VerificationKey) -> PreparedVerifyingKey {
    let gamma = vk.gamma_g2.neg();
    let delta = vk.delta_g2.neg();

    PreparedVerifyingKey {
        alpha_g1_beta_g2: BLS12_381Pairing::pairing(&vk.alpha_g1, &vk.beta_g2),
        neg_gamma_g2: gamma.into(),
        neg_delta_g2: delta.into(),
        ic: vk.ic.clone(),
    }
}
