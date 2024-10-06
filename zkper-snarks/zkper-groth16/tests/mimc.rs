use rand::Rng;
use zkper_base::rand::ZkperRng;
use zkper_curves::{
    curves::bls12_381::{
        curves::g1::G1Projective, Bls12_381BaseField, Bls12_381ScalarField, BLS12_381_BASE,
    },
    traits::field::FieldTrait,
};
// use zkper_curves::curves::bls12_381::Bls12_381ScalarField;

#[test]
fn test_mimc() {
    let mut rng = ZkperRng::new_test();

    let t = G1Projective::random_mont(&mut rng);
    let t = t.from_montgomery();
    println!("t: {:#}", t);

    // let g1 = G1Projective::random(&mut rng);
    // println!("g1: {:#}", g1.normalize());

    // let g2 = G1Projective::from_str_hex(
    //     "0x0a8477dd8aaa6a11b676a1dfadaa279b61a944ef9bf025fa4325adcefa057a13a083c85c8f0b4af860f92cc4e53701a1",
    //     "0x0ac9ea908703e3683863bbaa581dcd1448bbde65634c258cc935154ac2f276e35a10c6b87fa832f7d3a931161c81d958",
    //     "0x0ea1902baa6cd465a8ca51d8919f697413e2310b6671b4e3075929e4f46259ff3869e1f54557374c9a427b1678325240",
    // );

    // println!("g2: {:#}", g2.normalize());

    // println!("{}", g1 == g2);

    // let s: Bls12_381ScalarField = rng.gen();
    // let b: Bls12_381BaseField = rng.gen();

    // println!("s: {}", s);
    // println!("b: {}", b);

    // let s: Bls12_381ScalarField = rng.gen();
    // let b: Bls12_381BaseField = rng.gen();

    // println!("s: {}", s);
    // println!("b: {}", b);

    // let s = Scalar::random(&mut rng);
    // let b = Fp::random(&mut rng);

    // println!("s: {:?}", s);
    // println!("b: {:?}", b);

    // let s = Scalar::random(&mut rng);
    // let b = Fp::random(&mut rng);

    // println!("s: {:?}", s);
    // println!("b: {:?}", b);
}
