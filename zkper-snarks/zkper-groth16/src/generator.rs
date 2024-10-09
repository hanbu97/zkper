use anyhow::Result;
use rand::Rng;
use rand::RngCore;
use rug::Integer;
use zkper_curves::curves::bls12_381::{
    curves::{g1::G1Projective, g2::G2Projective},
    Bls12_381ScalarField,
};

use crate::circuit::Circuit;
use crate::constraints::linear_combination::LinearCombination;
use crate::constraints::ConstraintSystem;
use crate::constraints::Variable;

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
            tau: rng.gen(),
            alpha: rng.gen(),
            beta: rng.gen(),
            gamma: rng.gen(),
            delta: rng.gen(),
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

    Ok(())
}
