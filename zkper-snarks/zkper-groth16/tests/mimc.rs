use rand::Rng;
use zkper_base::rand::ZkperRng;
use zkper_curves::curves::bls12_381::{Bls12_381BaseField, Bls12_381ScalarField};
// use zkper_curves::curves::bls12_381::Bls12_381ScalarField;

#[test]
fn test_mimc() {
    // This may not be cryptographically safe, use
    // `OsRng` (for example) in production software.
    // let mut rng = thread_rng();
    let mut rng = ZkperRng::new_test();

    let s: Bls12_381ScalarField = rng.gen();
    let b: Bls12_381BaseField = rng.gen();

    println!("s: {}", s);
    println!("b: {}", b);

    let s: Bls12_381ScalarField = rng.gen();
    let b: Bls12_381BaseField = rng.gen();

    println!("s: {}", s);
    println!("b: {}", b);

    // let s = Scalar::random(&mut rng);
    // let b = Fp::random(&mut rng);

    // println!("s: {:?}", s);
    // println!("b: {:?}", b);

    // let s = Scalar::random(&mut rng);
    // let b = Fp::random(&mut rng);

    // println!("s: {:?}", s);
    // println!("b: {:?}", b);
}
