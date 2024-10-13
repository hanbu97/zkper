use std::time::{Duration, Instant};

use zkper_base::rand::ZkperRng;
use zkper_curves::{
    curves::bls12_381::{Bls12_381ScalarField, BLS12_381_SCALAR},
    traits::field::FieldTrait,
};
use zkper_groth16::{
    generator::generate_proving_parameters,
    prover::create_proof,
    verifier::{prepare_verifying_key, verify_proof},
};

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
    const SAMPLES: u32 = 10;
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

        {
            let start = Instant::now();
            // Create an instance of our circuit (with the
            // witness)
            let c = MiMCDemo {
                xl: Some(xl),
                xr: Some(xr),
                constants: &constants,
            };

            // Create a groth16 proof
            let proof = create_proof(c, &params, &mut rng).unwrap();
            total_proving += start.elapsed();

            // verify the proof
            let start = Instant::now();
            let verified = verify_proof(&pvk, &proof, &[image.0]).unwrap();
            println!("verified: {:#}", verified);
            total_verifying += start.elapsed();
        }
    }

    let proving_avg = total_proving / SAMPLES;
    let proving_avg =
        proving_avg.subsec_nanos() as f64 / 1_000_000_000f64 + (proving_avg.as_secs() as f64);

    let verifying_avg = total_verifying / SAMPLES;
    let verifying_avg =
        verifying_avg.subsec_nanos() as f64 / 1_000_000_000f64 + (verifying_avg.as_secs() as f64);

    println!("Average proving time: {:?} seconds", proving_avg);
    println!("Average verifying time: {:?} seconds", verifying_avg);
}
