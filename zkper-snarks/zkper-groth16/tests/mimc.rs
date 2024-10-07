use rand::Rng;
use zkper_base::rand::ZkperRng;
use zkper_curves::{
    curves::bls12_381::{
        curves::g1::G1Projective, Bls12_381BaseField, Bls12_381ScalarField, BLS12_381_BASE,
        BLS12_381_SCALAR,
    },
    traits::field::FieldTrait,
};
use zkper_groth16::generator::generate_random_parameters;

use crate::test_mimc::MiMCDemo;
// use zkper_curves::curves::bls12_381::Bls12_381ScalarField;

pub mod test_mimc;

pub const MIMC_ROUNDS: usize = 322;

#[test]
fn test_mimc() {
    let mut rng = ZkperRng::new_test();

    // Generate the MiMC round constants
    let constants = (0..MIMC_ROUNDS)
        .map(|_| BLS12_381_SCALAR.sample_raw(&mut rng))
        .collect::<Vec<_>>();

    // for c in &constants[..10] {
    //     println!("{}", c.to_string_radix(16));
    // }

    println!("Creating parameters...");

    // Create parameters for our circuit
    let c = MiMCDemo {
        xl: None,
        xr: None,
        constants: &constants,
    };

    let params = generate_random_parameters(c, &mut rng).unwrap();
}
