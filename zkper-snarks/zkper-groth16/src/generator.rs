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

/// Generates a random common reference string for
/// a circuit.
pub fn generate_random_parameters<C: Circuit, R: RngCore>(
    circuit: C,
    mut rng: &mut R,
) -> Result<()> {
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

    let gamma_inverse = BLS12_381_SCALAR.invert(toxic_waste.gamma.0).unwrap();
    let delta_inverse = BLS12_381_SCALAR.invert(toxic_waste.delta.0).unwrap();

    println!("gamma_inverse {}", gamma_inverse.to_string_radix(16));
    println!("delta_inverse {}", delta_inverse.to_string_radix(16));

    // Compute H query
    let mut h = vec![G1Projective::identity(); domain.coeffs.len() - 1];
    let mut coeff = domain.z(&toxic_waste.tau.0);
    coeff = BLS12_381_SCALAR.mul(coeff, &delta_inverse);

    for (he, de) in h.iter_mut().zip(&domain.coeffs) {
        let exp = BLS12_381_SCALAR.mul(de.clone(), &coeff);
        *he = g1.mul_scalar(&exp);
    }

    // Use inverse FFT to convert powers of tau to Lagrange coefficients

    Ok(())
}
