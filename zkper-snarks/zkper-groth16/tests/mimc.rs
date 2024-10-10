use std::time::{Duration, Instant};

use rand::Rng;
use zkper_base::rand::ZkperRng;
use zkper_curves::{
    curves::bls12_381::{
        curves::g1::G1Projective, Bls12_381BaseField, Bls12_381ScalarField, BLS12_381_BASE,
        BLS12_381_SCALAR,
    },
    traits::field::FieldTrait,
};
use zkper_groth16::{generator::generate_proving_parameters, verifier::prepare_verifying_key};

use crate::test_mimc::{implemention::mimc_implemention, MiMCDemo};
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

    let params = generate_proving_parameters(c, &mut rng).unwrap();

    // Prepare the verification key (for proof verification)
    let pvk = prepare_verifying_key(&params.vk);

    println!("Creating proofs...");

    // Let's benchmark stuff!
    const SAMPLES: u32 = 2;
    let mut total_proving = Duration::new(0, 0);
    let mut total_verifying = Duration::new(0, 0);

    // Just a place to put the proof data, so we can
    // benchmark deserialization.
    let mut proof_vec: Vec<u8> = vec![];

    let constants_scalar = constants
        .clone()
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<Bls12_381ScalarField>>();

    for _ in 0..SAMPLES {
        // Generate a random preimage and compute the image
        let xl = Bls12_381ScalarField::random(&mut rng);
        let xr = Bls12_381ScalarField::random(&mut rng);
        let image = mimc_implemention(xl.clone().into(), xr.clone().into(), &constants_scalar);

        proof_vec.truncate(0);

        let start = Instant::now();
        {
            // Create an instance of our circuit (with the
            // witness)
            let c = MiMCDemo {
                xl: Some(xl),
                xr: Some(xr),
                constants: &constants,
            };

            // Create a groth16 proof with our parameters.
            // let proof = create_random_proof(c, &params, &mut rng).unwrap();

            // proof.write(&mut proof_vec).unwrap();
        }
    }
}
