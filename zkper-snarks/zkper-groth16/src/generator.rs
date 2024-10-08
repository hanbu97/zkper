use anyhow::Result;
use rand::Rng;
use rand::RngCore;
use zkper_curves::curves::bls12_381::{
    curves::{g1::G1Projective, g2::G2Projective},
    Bls12_381ScalarField,
};

use crate::circuit::Circuit;

/// Generates a random common reference string for
/// a circuit.
pub fn generate_random_parameters<C: Circuit, R: RngCore>(
    circuit: C,
    mut rng: &mut R,
) -> Result<()> {
    let g1 = G1Projective::random_mont(&mut rng).from_montgomery();
    let g2 = G2Projective::random(&mut rng);
    let alpha: Bls12_381ScalarField = rng.gen();
    let beta: Bls12_381ScalarField = rng.gen();
    let gamma: Bls12_381ScalarField = rng.gen();
    let delta: Bls12_381ScalarField = rng.gen();
    let tau: Bls12_381ScalarField = rng.gen();

    // generate_parameters::<E, C>(circuit, g1, g2, alpha, beta, gamma, delta, tau)

    Ok(())
}
