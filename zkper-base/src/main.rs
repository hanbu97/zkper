use zkper_base::{
    integer::{backends::rug_backend::RugBackend, ZkperInteger},
    math::factorization::find_generator,
};

fn main() {
    let modulus = ZkperInteger::<RugBackend>::from_str(
        "52435875175126190479447740508185965837690552500527637822603658699938581184513",
    );

    let generator = find_generator(&modulus);
    println!("Generator: {}", generator);

    let two_adicity = (&modulus - ZkperInteger::one()).find_first_one(0).unwrap();
    let trace = (&modulus - 1) >> two_adicity;
    let two_adic_root_of_unity = generator.pow_mod(&trace, &modulus);
    println!("2-adic Root of Unity: {}", two_adic_root_of_unity);
}
