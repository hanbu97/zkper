use std::sync::Arc;

use anyhow::Result;
use rand::Rng;
use rand::RngCore;
use rug::Assign;
use rug::Integer;
use zkper_curves::curves::bls12_381::BLS12_381_SCALAR;
use zkper_curves::curves::bls12_381::{
    curves::{g1::G1Projective, g2::G2Projective},
    Bls12_381ScalarField,
};

use crate::circuit::Circuit;
use crate::constraints::linear_combination::LinearCombination;
use crate::constraints::ConstraintSystem;
use crate::constraints::Variable;
use crate::evaluation_domain::EvaluationDomain;
use crate::models::proving_parameters::ProvingParameters;
use crate::models::verification_key::VerificationKey;

pub struct ToxicWaste {
    pub alpha: Bls12_381ScalarField,
    pub beta: Bls12_381ScalarField,
    pub gamma: Bls12_381ScalarField,
    pub delta: Bls12_381ScalarField,
    pub tau: Bls12_381ScalarField,
}

impl ToxicWaste {
    pub fn sample<R: RngCore>(rng: &mut R) -> Self {
        Self {
            alpha: rng.gen(),
            beta: rng.gen(),
            gamma: rng.gen(),
            delta: rng.gen(),
            tau: rng.gen(),
        }
    }
}

/// Generates a random common reference string for a circuit.
pub fn generate_proving_parameters<C: Circuit, R: RngCore>(
    circuit: C,
    mut rng: &mut R,
) -> Result<ProvingParameters> {
    let g1 = G1Projective::random_mont(&mut rng).from_montgomery();
    let g2 = G2Projective::random(&mut rng);
    let toxic_waste = ToxicWaste::sample(&mut rng);

    let mut cs = ConstraintSystem::new();

    // Synthesize the circuit.
    circuit.synthesize(&mut cs)?;

    // Input constraints to ensure full density of IC query
    // x * 0 = 0
    for i in 0..cs.num_public_inputs {
        let a = LinearCombination::new_variable(Variable::Public(i));
        let b = LinearCombination::zero();
        let c = LinearCombination::zero();
        cs.enforce_constraint(a, b, c);
    }

    // Create bases for blind evaluation of polynomials at tau
    let powers_of_tau = vec![Integer::ZERO; cs.num_constraints];
    let mut domain = EvaluationDomain::new(powers_of_tau)?;

    // Compute powers of tau
    let mut current_tau_power = Integer::ONE.clone();
    for p in domain.coeffs.iter_mut() {
        p.assign(current_tau_power.clone());
        current_tau_power = BLS12_381_SCALAR.mul(current_tau_power, &toxic_waste.tau.0);
    }

    let gamma_inverse = BLS12_381_SCALAR
        .invert(toxic_waste.gamma.0.clone())
        .unwrap();
    let delta_inverse = BLS12_381_SCALAR
        .invert(toxic_waste.delta.0.clone())
        .unwrap();

    // Compute H query
    let mut h = vec![G1Projective::identity(); domain.coeffs.len() - 1];
    let mut coeff = domain.z(&toxic_waste.tau.0);
    coeff = BLS12_381_SCALAR.mul(coeff, &delta_inverse);

    for (he, de) in h.iter_mut().zip(&domain.coeffs) {
        let exp = BLS12_381_SCALAR.mul(de.clone(), &coeff);
        *he = g1.mul_scalar(&exp);
    }

    // Use inverse FFT to convert powers of tau to Lagrange coefficients
    domain.ifft();

    let powers_of_tau = domain.coeffs;

    // QAP A polynomial commitments for public and private variables
    let mut qap_a_commitments =
        vec![G1Projective::identity(); cs.num_public_inputs + cs.num_private_inputs];
    // QAP B polynomial commitments in G1 for public and private variables
    let mut qap_b_g1_commitments = qap_a_commitments.clone();
    // QAP B polynomial commitments in G2 for public and private variables
    let mut qap_b_g2_commitments =
        vec![G2Projective::identity(); cs.num_public_inputs + cs.num_private_inputs];
    // Commitments to public variables for efficient public input verification
    let mut public_commitments = vec![G1Projective::identity(); cs.num_public_inputs];
    // Commitments to private variables used in the proof
    let mut private_commitments = vec![G1Projective::identity(); cs.num_private_inputs];

    // Compute polynomial commitments for public inputs
    compute_polynomial_commitments(
        &powers_of_tau,
        &cs.at_public,
        &cs.bt_public,
        &cs.ct_public,
        &mut qap_a_commitments[0..cs.num_public_inputs],
        &mut qap_b_g1_commitments[0..cs.num_public_inputs],
        &mut qap_b_g2_commitments[0..cs.num_public_inputs],
        &mut public_commitments,
        &gamma_inverse,
        &toxic_waste.alpha.0,
        &toxic_waste.beta.0,
        g1.clone(),
        g2.clone(),
    );

    // Compute polynomial commitments for private inputs
    compute_polynomial_commitments(
        &powers_of_tau,
        &cs.at_private,
        &cs.bt_private,
        &cs.ct_private,
        &mut qap_a_commitments[cs.num_public_inputs..],
        &mut qap_b_g1_commitments[cs.num_public_inputs..],
        &mut qap_b_g2_commitments[cs.num_public_inputs..],
        &mut private_commitments,
        &delta_inverse,
        &toxic_waste.alpha.0,
        &toxic_waste.beta.0,
        g1.clone(),
        g2.clone(),
    );

    // Ensure all private variable commitments are non-zero
    for e in private_commitments.iter() {
        if e.is_identity() {
            return Err(anyhow::anyhow!("Unconstrained variable"));
        }
    }

    // Create verification key
    let vk = VerificationKey {
        alpha_g1: g1.mul_scalar(&toxic_waste.alpha.0).to_affine(),
        beta_g1: g1.mul_scalar(&toxic_waste.beta.0).to_affine(),
        beta_g2: g2.mul_scalar(&toxic_waste.beta.0).to_affine(),
        gamma_g2: g2.mul_scalar(&toxic_waste.gamma.0).to_affine(),
        delta_g1: g1.mul_scalar(&toxic_waste.delta.0).to_affine(),
        delta_g2: g2.mul_scalar(&toxic_waste.delta.0).to_affine(),
        ic: public_commitments.iter().map(|e| e.to_affine()).collect(),
    };

    // Create proving parameters
    let pk = ProvingParameters {
        vk: vk.clone(),
        h_query: Arc::new(h.iter().map(|e| e.to_affine()).collect()),
        l_query: Arc::new(private_commitments.iter().map(|e| e.to_affine()).collect()),
        a_query: Arc::new(
            qap_a_commitments
                .into_iter()
                .filter(|e| !e.is_identity())
                .map(|e| e.to_affine())
                .collect(),
        ),
        b_g1_query: Arc::new(
            qap_b_g1_commitments
                .into_iter()
                .filter(|e| !e.is_identity())
                .map(|e| e.to_affine())
                .collect(),
        ),
        b_g2_query: Arc::new(
            qap_b_g2_commitments
                .into_iter()
                .filter(|e| !e.is_identity())
                .map(|e| e.to_affine())
                .collect(),
        ),
    };

    Ok(pk)
}

fn compute_polynomial_commitments(
    powers_of_tau: &[Integer],
    at: &[Vec<(Integer, usize)>],
    bt: &[Vec<(Integer, usize)>],
    ct: &[Vec<(Integer, usize)>],
    a: &mut [G1Projective],
    b_g1: &mut [G1Projective],
    b_g2: &mut [G2Projective],
    ext: &mut [G1Projective],
    inv: &Integer,
    alpha: &Integer,
    beta: &Integer,
    g1: G1Projective,
    g2: G2Projective,
) {
    for ((((((a, b_g1), b_g2), ext), at), bt), ct) in a
        .iter_mut()
        .zip(b_g1.iter_mut())
        .zip(b_g2.iter_mut())
        .zip(ext.iter_mut())
        .zip(at.iter())
        .zip(bt.iter())
        .zip(ct.iter())
    {
        let eval_at_tau = |p: &[(Integer, usize)]| {
            p.iter().fold(Integer::ZERO, |mut acc, (coeff, index)| {
                let mut term = powers_of_tau[*index].clone();
                term = BLS12_381_SCALAR.mul(term, coeff);
                acc = BLS12_381_SCALAR.add(acc, &term);
                acc
            })
        };

        let mut at_eval = eval_at_tau(at);
        let mut bt_eval = eval_at_tau(bt);
        let ct_eval = eval_at_tau(ct);

        *a = g1.mul_scalar(&at_eval);
        *b_g1 = g1.mul_scalar(&bt_eval);
        *b_g2 = g2.mul_scalar(&bt_eval);

        at_eval = BLS12_381_SCALAR.mul(at_eval, beta);
        bt_eval = BLS12_381_SCALAR.mul(bt_eval, alpha);

        let mut e = BLS12_381_SCALAR.add(at_eval, &bt_eval);
        e = BLS12_381_SCALAR.add(e, &ct_eval);
        e = BLS12_381_SCALAR.mul(e, inv);

        *ext = g1.mul_scalar(&e);
    }
}
