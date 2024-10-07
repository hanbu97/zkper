use anyhow::Result;
use rand::RngCore;
use zkper_curves::curves::bls12_381::curves::g1::G1Projective;

use crate::circuit::Circuit;

/// Generates a random common reference string for
/// a circuit.
pub fn generate_random_parameters<C: Circuit, R: RngCore>(
    circuit: C,
    mut rng: &mut R,
) -> Result<()> {
    let g1 = G1Projective::random_mont(&mut rng).from_montgomery();

    println!("-----------{:#}", g1);
    // let g2 = E::G2::random(&mut rng);
    // let alpha = E::Fr::random(&mut rng);
    // let beta = E::Fr::random(&mut rng);
    // let gamma = E::Fr::random(&mut rng);
    // let delta = E::Fr::random(&mut rng);
    // let tau = E::Fr::random(&mut rng);

    // generate_parameters::<E, C>(circuit, g1, g2, alpha, beta, gamma, delta, tau)

    Ok(())
}
