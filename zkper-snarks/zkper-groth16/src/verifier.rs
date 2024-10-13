use rug::Integer;
use zkper_curves::curves::bls12_381::paring::BLS12_381Pairing;

use crate::models::{
    proof::Proof, verification_key::VerificationKey, verify::PreparedVerifyingKey,
};

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

pub fn verify_proof(
    pvk: &PreparedVerifyingKey,
    proof: &Proof,
    public_inputs: &[Integer],
) -> anyhow::Result<bool> {
    if (public_inputs.len() + 1) != pvk.ic.len() {
        return Err(anyhow::anyhow!("InvalidVerifyingKey"));
    }

    let mut acc = pvk.ic[0].to_curve();
    for (public_input, b) in public_inputs.iter().zip(pvk.ic.iter().skip(1)) {
        acc = acc.add(&b.to_curve().mul_scalar(public_input));
    }

    let answer = BLS12_381Pairing::multi_miller_loop(&[
        (&proof.a, &proof.b.clone().into()),
        (&acc.to_affine(), &pvk.neg_gamma_g2),
        (&proof.c, &pvk.neg_delta_g2),
    ]);

    let answer = BLS12_381Pairing::final_exponentiation(&answer);
    let is_equal = answer == pvk.alpha_g1_beta_g2;

    Ok(is_equal)
}
